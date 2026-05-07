//! Buttons style

#![allow(clippy::module_name_repetitions)]

use iced::border::Radius;
use iced::widget::button;
use iced::widget::button::{Catalog, Status, Style};
use iced::{Background, Border, Color, Shadow, Vector};

use crate::StyleType;
use crate::gui::styles::style_constants::{BORDER_BUTTON_RADIUS, BORDER_WIDTH};
use crate::gui::styles::types::gradient_type::{
    GradientType, get_gradient_buttons, get_gradient_hovered_buttons,
};
use crate::gui::styles::types::palette::mix_colors;

#[derive(Default)]
pub enum ButtonType {
    #[default]
    Standard,
    AdapterCard,
    AdapterCardSelected,
    StartPageOption,
    BorderedRound,
    BorderedRoundSelected,
    TabActive,
    TabInactive,
    Starred,
    NotStarred,
    Neutral,
    Alert,
    Start(GradientType),
    IconAction,
    SortArrows,
    SortArrowActive,
    Thumbnail,
}

impl ButtonType {
    fn rounded_radius(&self) -> Radius {
        match self {
            ButtonType::Neutral => 0.0.into(),
            ButtonType::TabActive | ButtonType::TabInactive => Radius::new(0).bottom(30),
            ButtonType::AdapterCard
            | ButtonType::AdapterCardSelected
            | ButtonType::StartPageOption
            | ButtonType::BorderedRound
            | ButtonType::BorderedRoundSelected
            | ButtonType::Start(_) => 12.0.into(),
            ButtonType::Starred | ButtonType::NotStarred | ButtonType::IconAction => 100.0.into(),
            _ => BORDER_BUTTON_RADIUS.into(),
        }
    }

    fn active(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        button::Style {
            background: Some(match self {
                ButtonType::AdapterCard => Background::Color(Color {
                    a: ext.alpha_round_containers,
                    ..ext.buttons_color
                }),
                ButtonType::AdapterCardSelected => Background::Color(Color {
                    a: if ext.is_nightly { 0.24 } else { 0.38 },
                    ..ext.buttons_color
                }),
                ButtonType::StartPageOption => Background::Color(Color {
                    a: if ext.is_nightly { 0.16 } else { 0.28 },
                    ..ext.buttons_color
                }),
                ButtonType::TabActive | ButtonType::BorderedRoundSelected => {
                    Background::Color(mix_colors(colors.primary, ext.buttons_color))
                }
                ButtonType::BorderedRound => Background::Color(Color {
                    a: ext.alpha_round_containers,
                    ..ext.buttons_color
                }),
                ButtonType::Neutral
                | ButtonType::Thumbnail
                | ButtonType::Starred
                | ButtonType::NotStarred
                | ButtonType::SortArrows
                | ButtonType::SortArrowActive => Background::Color(Color::TRANSPARENT),
                ButtonType::Start(GradientType::None) => {
                    Background::Color(mix_colors(colors.primary, colors.secondary))
                }
                ButtonType::Start(gradient_type) => Background::Gradient(get_gradient_buttons(
                    &colors,
                    *gradient_type,
                    ext.is_nightly,
                    0.84,
                )),
                ButtonType::IconAction => Background::Color(Color {
                    a: if ext.is_nightly { 0.55 } else { 0.70 },
                    ..ext.buttons_color
                }),
                _ => Background::Color(ext.buttons_color),
            }),
            border: Border {
                radius: self.rounded_radius(),
                width: match self {
                    ButtonType::TabActive
                    | ButtonType::TabInactive
                    | ButtonType::SortArrows
                    | ButtonType::SortArrowActive
                    | ButtonType::Starred
                    | ButtonType::NotStarred
                    | ButtonType::Neutral
                    | ButtonType::Thumbnail => 0.0,
                    ButtonType::AdapterCard
                    | ButtonType::AdapterCardSelected
                    | ButtonType::StartPageOption => BORDER_WIDTH / 2.0,
                    ButtonType::BorderedRound => BORDER_WIDTH * 2.0,
                    _ => BORDER_WIDTH,
                },
                color: match self {
                    ButtonType::Alert => ext.red_alert_color,
                    ButtonType::AdapterCard => Color {
                        a: ext.alpha_round_borders * 0.75,
                        ..ext.buttons_color
                    },
                    ButtonType::AdapterCardSelected => Color {
                        a: if ext.is_nightly { 0.58 } else { 0.72 },
                        ..colors.secondary
                    },
                    ButtonType::StartPageOption => Color {
                        a: ext.alpha_round_borders * 0.9,
                        ..ext.buttons_color
                    },
                    ButtonType::BorderedRound => Color {
                        a: ext.alpha_round_borders,
                        ..ext.buttons_color
                    },
                    ButtonType::Start(_) => Color {
                        a: 0.58,
                        ..colors.secondary
                    },
                    ButtonType::IconAction => Color {
                        a: if ext.is_nightly { 0.38 } else { 0.52 },
                        ..colors.secondary
                    },
                    _ => colors.secondary,
                },
            },
            text_color: match self {
                ButtonType::Starred => colors.starred,
                ButtonType::SortArrows => Color {
                    a: ext.alpha_chart_badge,
                    ..colors.text_body
                },
                ButtonType::SortArrowActive => colors.secondary,
                ButtonType::Start(_) => colors.text_headers,
                ButtonType::Thumbnail | ButtonType::IconAction => {
                    mix_colors(colors.text_headers, colors.secondary)
                }
                _ => colors.text_body,
            },
            shadow: match self {
                ButtonType::AdapterCardSelected | ButtonType::StartPageOption => Shadow {
                    color: Color {
                        a: 0.16,
                        ..colors.secondary
                    },
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 6.0,
                },
                ButtonType::AdapterCard | ButtonType::Start(_) | ButtonType::IconAction => Shadow {
                    color: Color {
                        a: 0.24,
                        ..Color::BLACK
                    },
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 4.0,
                },
                ButtonType::TabActive | ButtonType::TabInactive => Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(3.0, 2.0),
                    blur_radius: 4.0,
                },
                _ => Shadow::default(),
            },
            snap: true,
        }
    }

    fn hovered(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        button::Style {
            shadow: match self {
                ButtonType::Neutral
                | ButtonType::SortArrows
                | ButtonType::SortArrowActive
                | ButtonType::Thumbnail => Shadow::default(),
                _ => Shadow {
                    color: Color::BLACK,
                    offset: match self {
                        ButtonType::TabActive | ButtonType::TabInactive => Vector::new(3.0, 3.0),
                        _ => Vector::new(0.0, 2.0),
                    },
                    blur_radius: match self {
                        ButtonType::TabActive | ButtonType::TabInactive => 4.0,
                        _ => 2.0,
                    },
                },
            },
            background: Some(match self {
                ButtonType::AdapterCard => Background::Color(Color {
                    a: if ext.is_nightly { 0.24 } else { 0.38 },
                    ..ext.buttons_color
                }),
                ButtonType::AdapterCardSelected => Background::Color(Color {
                    a: if ext.is_nightly { 0.32 } else { 0.48 },
                    ..ext.buttons_color
                }),
                ButtonType::StartPageOption => Background::Color(Color {
                    a: if ext.is_nightly { 0.28 } else { 0.42 },
                    ..ext.buttons_color
                }),
                ButtonType::SortArrows | ButtonType::SortArrowActive | ButtonType::Thumbnail => {
                    Background::Color(Color::TRANSPARENT)
                }
                ButtonType::Neutral => Background::Color(Color {
                    a: ext.alpha_round_borders,
                    ..ext.buttons_color
                }),
                ButtonType::Start(GradientType::None) => {
                    Background::Color(mix_colors(colors.primary, colors.secondary))
                }
                ButtonType::Start(gradient_type) => Background::Gradient(
                    get_gradient_hovered_buttons(&colors, *gradient_type, ext.is_nightly),
                ),
                ButtonType::BorderedRoundSelected => Background::Color(ext.buttons_color),
                ButtonType::IconAction => Background::Color(Color {
                    a: if ext.is_nightly { 0.78 } else { 0.88 },
                    ..ext.buttons_color
                }),
                _ => Background::Color(mix_colors(colors.primary, ext.buttons_color)),
            }),
            border: Border {
                radius: self.rounded_radius(),
                width: match self {
                    ButtonType::Starred
                    | ButtonType::NotStarred
                    | ButtonType::TabActive
                    | ButtonType::SortArrows
                    | ButtonType::SortArrowActive
                    | ButtonType::TabInactive
                    | ButtonType::Thumbnail
                    | ButtonType::BorderedRound => 0.0,
                    ButtonType::AdapterCard
                    | ButtonType::AdapterCardSelected
                    | ButtonType::StartPageOption => BORDER_WIDTH / 2.0,
                    _ => BORDER_WIDTH,
                },
                color: match self {
                    ButtonType::Alert => ext.red_alert_color,
                    ButtonType::AdapterCard => Color {
                        a: ext.alpha_round_borders,
                        ..ext.buttons_color
                    },
                    ButtonType::AdapterCardSelected | ButtonType::Start(_) => Color {
                        a: 0.76,
                        ..colors.secondary
                    },
                    ButtonType::StartPageOption => Color {
                        a: if ext.is_nightly { 0.42 } else { 0.58 },
                        ..colors.secondary
                    },
                    ButtonType::BorderedRound => Color {
                        a: ext.alpha_round_borders,
                        ..ext.buttons_color
                    },
                    ButtonType::Neutral => ext.buttons_color,
                    ButtonType::IconAction => Color {
                        a: if ext.is_nightly { 0.52 } else { 0.68 },
                        ..colors.secondary
                    },
                    _ => colors.secondary,
                },
            },
            text_color: match self {
                ButtonType::Starred => colors.starred,
                ButtonType::Start(_) | ButtonType::Thumbnail | ButtonType::IconAction => {
                    colors.text_headers
                }
                ButtonType::SortArrowActive | ButtonType::SortArrows => colors.secondary,
                _ => colors.text_body,
            },
            snap: true,
        }
    }

    fn pressed(&self, style: &StyleType) -> Style {
        let mut active = self.active(style);
        active.shadow = Shadow {
            color: Color {
                a: 0.18,
                ..Color::BLACK
            },
            offset: Vector::new(0.0, 0.5),
            blur_radius: 2.0,
        };
        if matches!(
            self,
            ButtonType::AdapterCard
                | ButtonType::AdapterCardSelected
                | ButtonType::StartPageOption
                | ButtonType::Start(_)
                | ButtonType::IconAction
        ) {
            active.background = self.hovered(style).background;
        }
        active
    }

    fn disabled(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        match self {
            ButtonType::Start(_) => {
                let mut disabled = self.active(style);
                disabled.background = Some(match self {
                    ButtonType::Start(GradientType::None) => Background::Color(Color {
                        a: ext.alpha_chart_badge,
                        ..colors.secondary
                    }),
                    ButtonType::Start(gradient_type) => Background::Gradient(get_gradient_buttons(
                        &colors,
                        *gradient_type,
                        ext.is_nightly,
                        ext.alpha_chart_badge,
                    )),
                    _ => Background::Color(ext.buttons_color),
                });
                disabled.text_color = Color {
                    a: 0.5,
                    ..colors.text_headers
                };
                disabled.shadow = Shadow::default();
                disabled
            }
            ButtonType::Standard => Style {
                background: Some(Background::Color(Color {
                    a: ext.alpha_chart_badge,
                    ..ext.buttons_color
                })),
                border: Border {
                    radius: BORDER_BUTTON_RADIUS.into(),
                    width: BORDER_WIDTH,
                    color: Color {
                        a: ext.alpha_chart_badge,
                        ..colors.secondary
                    },
                },
                text_color: Color {
                    a: ext.alpha_chart_badge,
                    ..colors.text_body
                },
                shadow: Shadow::default(),
                snap: true,
            },
            _ => self.active(style),
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = ButtonType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active => class.active(self),
            Status::Pressed => class.pressed(self),
            Status::Hovered => class.hovered(self),
            Status::Disabled => class.disabled(self),
        }
    }
}
