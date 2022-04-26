use iced::{button, Background, Color, Font, Vector};

pub const OPEN_SANS: Font = Font::External {
    name: "Open Sans",
    bytes: include_bytes!("../fonts/OpenSans-Regular.ttf"),
};

pub const OPEN_SANS_BOLD: Font = Font::External {
    name: "Open Sans Bold",
    bytes: include_bytes!("../fonts/OpenSans-Bold.ttf"),
};

pub const ACCENT_COLOR: Color = Color::from_rgb(0.0, 0.76, 0.04);
pub const RED: Color = Color::from_rgb(1.0, 0.0, 0.0);
pub const DARK_GREY: Color = Color::from_rgb(0.3, 0.3, 0.3);
pub const GREY: Color = Color::from_rgb(0.6, 0.6, 0.6);

pub enum Button {
    Deselected,
    Confirm,
    Icon,
    IconDestructive,
    Destructive,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::Deselected => button::Style::default(),
            Button::Confirm => button::Style {
                background: Some(Background::Color(ACCENT_COLOR)),
                border_radius: 3.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
            Button::Icon => button::Style {
                text_color: ACCENT_COLOR,
                ..button::Style::default()
            },
            Button::IconDestructive => button::Style {
                text_color: Color::from_rgb(1.0, 0.0, 0.0),
                ..button::Style::default()
            },
            Button::Destructive => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                border_radius: 3.0,
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 1.0),
                ..button::Style::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            text_color: match self {
                Button::Icon => ACCENT_COLOR,
                Button::Deselected => ACCENT_COLOR,
                _ => active.text_color,
            },
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..active
        }
    }
}

pub enum SidebarButton {
    Deselected,
    Selected,
}

impl button::StyleSheet for SidebarButton {
    fn active(&self) -> button::Style {
        match self {
            SidebarButton::Deselected => button::Style::default(),
            SidebarButton::Selected => button::Style {
                background: Some(Background::Color(ACCENT_COLOR)),
                border_radius: 0.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            text_color: match self {
                SidebarButton::Deselected => ACCENT_COLOR,
                _ => active.text_color,
            },
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..active
        }
    }
}

pub struct TabContents;

impl iced::container::StyleSheet for TabContents {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
            ..iced::container::Style::default()
        }
    }
}

pub struct EditPane;

impl iced::container::StyleSheet for EditPane {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            background: Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
            ..iced::container::Style::default()
        }
    }
}

pub enum TableRow {
    Header,
    Darker,
    Lighter,
}

impl iced::container::StyleSheet for TableRow {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            text_color: Some(match self {
                TableRow::Header => Color::WHITE,
                _ => Color::BLACK,
            }),
            background: Some(Background::Color(match self {
                TableRow::Darker => Color::from_rgb(0.975, 0.975, 0.975),
                TableRow::Lighter => Color::from_rgb(1.0, 1.0, 1.0),
                TableRow::Header => Color::from_rgb(0.2, 0.2, 0.2),
            })),
            ..iced::container::Style::default()
        }
    }
}
