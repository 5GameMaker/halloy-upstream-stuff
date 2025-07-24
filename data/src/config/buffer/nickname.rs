use serde::Deserialize;

use crate::buffer::{Alignment, Brackets, Color};
use crate::config::buffer::{Away, NicknameClickAction};

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Nickname {
    pub away: Away,
    pub offline: Offline,
    pub color: Color,
    pub brackets: Brackets,
    pub alignment: Alignment,
    pub show_access_levels: bool,
    pub click: NicknameClickAction,
}

impl Default for Nickname {
    fn default() -> Self {
        Self {
            away: Away::default(),
            offline: Offline::default(),
            color: Color::default(),
            brackets: Brackets::default(),
            alignment: Alignment::default(),
            show_access_levels: true,
            click: NicknameClickAction::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Offline {
    #[default]
    Solid,
    None,
}

impl Offline {
    pub fn is_offline(&self, is_user_offline: bool) -> bool {
        is_user_offline && matches!(self, Offline::Solid)
    }
}
