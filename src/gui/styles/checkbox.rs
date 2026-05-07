//! Checkbox style

#![allow(clippy::module_name_repetitions)]

use iced::widget::checkbox::{Catalog, Status, Style};
use iced::{Background, Border, Color};

use crate::StyleType;
use crate::gui::styles::style_constants::BORDER_WIDTH;

#[derive(Default)]
pub enum CheckboxType {
    #[default]
    Standard,
}

const CHECKBOX_BORDER_RADIUS: f32 = 5.0;

impl CheckboxType {
    #[allow(clippy::unused_self)]
    fn active(&self, style: &StyleType, is_checked: bool) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            background: Background::Color(if is_checked {
                ext.buttons_color
            } else {
                Color {
                    a: ext.alpha_round_containers,
                    ..ext.buttons_color
                }
            }),
            icon_color: colors.text_body,
            border: Border {
                radius: CHECKBOX_BORDER_RADIUS.into(),
                width: if is_checked {
                    BORDER_WIDTH
                } else {
                    BORDER_WIDTH / 2.0
                },
                color: if is_checked {
                    colors.secondary
                } else {
                    Color {
                        a: ext.alpha_round_borders,
                        ..ext.buttons_color
                    }
                },
            },
            text_color: None,
        }
    }

    #[allow(clippy::unused_self)]
    fn hovered(&self, style: &StyleType, is_checked: bool) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            background: Background::Color(Color {
                a: if is_checked { 1.0 } else { 0.35 },
                ..ext.buttons_color
            }),
            icon_color: colors.text_body,
            border: Border {
                radius: CHECKBOX_BORDER_RADIUS.into(),
                width: BORDER_WIDTH,
                color: colors.secondary,
            },
            text_color: None,
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = CheckboxType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active { is_checked } | Status::Disabled { is_checked } => {
                class.active(self, is_checked)
            }
            Status::Hovered { is_checked } => class.hovered(self, is_checked),
        }
    }
}
