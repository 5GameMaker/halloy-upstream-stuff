use data::appearance::theme::{FontStyle, randomize_color};
use data::{Config, isupport, message, target};
use iced::widget::span;
use iced::widget::text::Span;
use iced::{Length, border};

use super::{Element, Renderer, selectable_rich_text, selectable_text};
use crate::{Theme, font};

pub fn message_content<'a, M: 'a>(
    content: &'a message::Content,
    casemapping: isupport::CaseMap,
    theme: &'a Theme,
    on_link: impl Fn(message::Link) -> M + 'a,
    style: impl Fn(&Theme) -> selectable_text::Style + 'a,
    font_style: impl Fn(&Theme) -> FontStyle,
    config: &Config,
) -> Element<'a, M> {
    message_content_impl::<(), M>(
        content,
        casemapping,
        theme,
        on_link,
        style,
        font_style,
        Option::<(fn(&message::Link) -> _, fn(&message::Link, _, _) -> _)>::None,
        config,
    )
}

pub fn with_context<'a, T: Copy + 'a, M: 'a>(
    content: &'a message::Content,
    casemapping: isupport::CaseMap,
    theme: &'a Theme,
    on_link: impl Fn(message::Link) -> M + 'a,
    style: impl Fn(&Theme) -> selectable_text::Style + 'a,
    font_style: impl Fn(&Theme) -> FontStyle,
    link_entries: impl Fn(&message::Link) -> Vec<T> + 'a,
    entry: impl Fn(&message::Link, T, Length) -> Element<'a, M> + 'a,
    config: &Config,
) -> Element<'a, M> {
    message_content_impl(
        content,
        casemapping,
        theme,
        on_link,
        style,
        font_style,
        Some((link_entries, entry)),
        config,
    )
}

#[allow(clippy::type_complexity)]
fn message_content_impl<'a, T: Copy + 'a, M: 'a>(
    content: &'a message::Content,
    casemapping: isupport::CaseMap,
    theme: &'a Theme,
    on_link: impl Fn(message::Link) -> M + 'a,
    style: impl Fn(&Theme) -> selectable_text::Style + 'a,
    font_style: impl Fn(&Theme) -> FontStyle,
    context_menu: Option<(
        impl Fn(&message::Link) -> Vec<T> + 'a,
        impl Fn(&message::Link, T, Length) -> Element<'a, M> + 'a,
    )>,
    config: &Config,
) -> Element<'a, M> {
    match content {
        data::message::Content::Plain(text) => selectable_text(text)
            .font_maybe(font::get(font_style(theme)))
            .style(style)
            .into(),
        data::message::Content::Fragments(fragments) => {
            let mut text = selectable_rich_text::<
                M,
                message::Link,
                T,
                Theme,
                Renderer,
            >(
                fragments
                    .iter()
                    .map(|fragment| match fragment {
                        data::message::Fragment::Text(s) => span(s),
                        data::message::Fragment::Channel(s) => span(s.as_str())
                            .font_maybe(font::get(
                                theme.styles().buffer.url.font_style,
                            ))
                            .color(theme.styles().buffer.url.color)
                            .link(message::Link::Channel(
                                target::Channel::from_str(
                                    s.as_str(),
                                    casemapping,
                                ),
                            )),
                        data::message::Fragment::User(user, text) => {
                            let color = theme.styles().buffer.nickname.color;
                            let seed = match &config
                                .buffer
                                .channel
                                .message
                                .nickname_color
                            {
                                data::buffer::Color::Solid => None,
                                data::buffer::Color::Unique => {
                                    Some(user.seed())
                                }
                            };

                            let color = match seed {
                                Some(seed) => randomize_color(color, seed),
                                None => theme.styles().text.primary.color,
                            };

                            span(text)
                                .font_maybe(font::get(
                                    theme.styles().buffer.nickname.font_style,
                                ))
                                .color(color)
                                .link(message::Link::User(user.clone()))
                        }
                        data::message::Fragment::HighlightNick(user, text) => {
                            let color = theme.styles().buffer.nickname.color;
                            let seed = match &config
                                .buffer
                                .channel
                                .message
                                .nickname_color
                            {
                                data::buffer::Color::Solid => None,
                                data::buffer::Color::Unique => {
                                    Some(user.seed())
                                }
                            };

                            let color = match seed {
                                Some(seed) => randomize_color(color, seed),
                                None => theme.styles().text.primary.color,
                            };

                            span(text)
                                .font_maybe(font::get(
                                    theme.styles().buffer.nickname.font_style,
                                ))
                                .color(color)
                                .background(theme.styles().buffer.highlight)
                                .link(message::Link::User(user.clone()))
                        }
                        data::message::Fragment::HighlightMatch(text) => {
                            span(text.as_str())
                                .font_maybe(font::get(
                                    theme.styles().text.primary.font_style,
                                ))
                                .color(theme.styles().text.primary.color)
                                .background(theme.styles().buffer.highlight)
                        }
                        data::message::Fragment::Url(s) => span(s.as_str())
                            .font_maybe(font::get(
                                theme.styles().buffer.url.font_style,
                            ))
                            .color(theme.styles().buffer.url.color)
                            .link(message::Link::Url(s.as_str().to_string())),
                        data::message::Fragment::Formatted {
                            text,
                            formatting,
                        } => {
                            let mut span = span(text)
                                .color_maybe(formatting.fg.and_then(|color| {
                                    color.into_iced(theme.styles())
                                }))
                                .background_maybe(formatting.bg.and_then(
                                    |color| color.into_iced(theme.styles()),
                                ))
                                .underline(formatting.underline)
                                .strikethrough(formatting.strikethrough);

                            let mut formatted_style = FontStyle::new(
                                formatting.bold,
                                formatting.italics,
                            );

                            if formatting.monospace {
                                span = span
                                    .padding([0, 4])
                                    .color(theme.styles().buffer.code.color)
                                    .border(
                                        border::rounded(3)
                                            .color(
                                                theme.styles().general.border,
                                            )
                                            .width(1),
                                    );

                                formatted_style = formatted_style
                                    + theme.styles().buffer.code.font_style;
                            } else {
                                formatted_style =
                                    formatted_style + font_style(theme);
                            }

                            span.font_maybe(font::get(formatted_style))
                        }
                    })
                    .collect::<Vec<_>>(),
            )
            .on_link(on_link)
            .font_maybe(font::get(font_style(theme)))
            .style(style);

            if let Some((link_entries, view)) = context_menu {
                text = text.context_menu(link_entries, view);
            }

            text.into()
        }
        data::message::Content::Log(record) => {
            let spans: Vec<Span<'a, message::Link, _>> =
                vec![span(&record.message)];

            selectable_rich_text::<M, message::Link, T, Theme, Renderer>(spans)
                .style(style)
                .into()
        }
    }
}
