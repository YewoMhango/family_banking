use iced::{button, text_input, Button, Column, Container, Text, TextInput};

use crate::{
    family_banking::Message,
    style::{self, ACCENT_COLOR, OPEN_SANS, OPEN_SANS_BOLD},
    WINDOW_HEIGHT,
};

#[derive(Debug, Clone, Default)]
pub struct LoginView {
    pub password_input_1_state: text_input::State,
    pub password_input_2_state: text_input::State,
    pub password_input_1_value: String,
    pub password_input_2_value: String,
    pub login_button_state: button::State,
    pub login_error_message: String,
}

pub fn render_login_view<'a>(login_view_data: &'a mut LoginView) -> Column<'a, Message> {
    Column::new()
        .width(iced::Length::Units(700))
        .height(iced::Length::Units(WINDOW_HEIGHT))
        .padding(150)
        .align_items(iced::Align::Center)
        .push(
            Text::new("Login")
                .font(OPEN_SANS_BOLD)
                .size(40)
                .color(ACCENT_COLOR),
        )
        .push(
            Container::new(
                TextInput::new(
                    &mut login_view_data.password_input_1_state,
                    "Password",
                    login_view_data.password_input_1_value.as_str(),
                    Message::PasswordInput1Changed,
                )
                .width(iced::Length::Units(300))
                .font(OPEN_SANS)
                .padding(10)
                .password(),
            )
            .padding(20),
        )
        .push(
            Text::new(login_view_data.login_error_message.clone())
                .font(OPEN_SANS)
                .color(style::RED),
        )
        .push(
            Button::new(
                &mut login_view_data.login_button_state,
                Text::new("Login").font(OPEN_SANS),
            )
            .style(style::Button::Confirm)
            .padding(10)
            .on_press(Message::LoginButtonPressed),
        )
}

pub fn render_new_password_view<'a>(login_view_data: &'a mut LoginView) -> Column<'a, Message> {
    Column::new()
        .width(iced::Length::Units(700))
        .height(iced::Length::Units(WINDOW_HEIGHT))
        .padding(150)
        .align_items(iced::Align::Center)
        .push(
            Text::new("Create Password")
                .font(OPEN_SANS_BOLD)
                .size(40)
                .color(ACCENT_COLOR),
        )
        .push(
            Column::new()
                .padding(20)
                .push(
                    TextInput::new(
                        &mut login_view_data.password_input_1_state,
                        "New password",
                        login_view_data.password_input_1_value.as_str(),
                        Message::PasswordInput1Changed,
                    )
                    .width(iced::Length::Units(300))
                    .font(OPEN_SANS)
                    .padding(10)
                    .password(),
                )
                .push(
                    TextInput::new(
                        &mut login_view_data.password_input_2_state,
                        "Confirm password",
                        login_view_data.password_input_2_value.as_str(),
                        Message::PasswordInput2Changed,
                    )
                    .width(iced::Length::Units(300))
                    .font(OPEN_SANS)
                    .padding(10)
                    .password(),
                ),
        )
        .push(
            Text::new(login_view_data.login_error_message.clone())
                .font(OPEN_SANS)
                .color(iced::Color::from_rgb(0.7, 0.0, 0.0)),
        )
        .push(
            Button::new(
                &mut login_view_data.login_button_state,
                Text::new("Confirm").font(OPEN_SANS),
            )
            .style(style::Button::Confirm)
            .padding(10)
            .on_press(Message::NewPasswordButtonPressed),
        )
}
