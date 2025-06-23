use iced::{
    Length, alignment,
    widget::{button, column, container, text, vertical_space},
};

use super::Message;
use crate::{theme, widget::Element};

pub fn view(payload: &str) -> Element<Message> {
    container(
        column![
            column![
                text("This hyperlink will take you to"),
                text(payload)
                    .style(theme::text::url)
                    .wrapping(text::Wrapping::Glyph)
                    .width(Length::Shrink),
                vertical_space().height(8),
                text("Are you sure you want to go there?"),
            ]
            .align_x(iced::Alignment::Center)
            .spacing(2),
            column![
                button(
                    container(text("Open URL"))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill),
                )
                .padding(5)
                .width(Length::Fixed(250.0))
                .style(|theme, status| theme::button::secondary(
                    theme, status, false
                ))
                .on_press(Message::OpenURL(payload.to_string())),
                button(
                    container(text("Close"))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill),
                )
                .padding(5)
                .width(Length::Fixed(250.0))
                .style(|theme, status| theme::button::secondary(
                    theme, status, false
                ))
                .on_press(Message::Cancel),
            ]
            .spacing(4),
        ]
        .spacing(20)
        .align_x(iced::Alignment::Center),
    )
    .max_width(400)
    .width(Length::Shrink)
    .style(theme::container::tooltip)
    .padding(25)
    .into()
}
