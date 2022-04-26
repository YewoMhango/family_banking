use iced::{button, text_input, Button, Column, Container, Row, Text, TextInput};

use crate::{
    family_banking::Message,
    format_decimal,
    style::{self, OPEN_SANS, OPEN_SANS_BOLD},
    EDIT_PANE_WIDTH, SIDEBAR_WIDTH, WINDOW_WIDTH,
};

use super::{render_edit_pane, EditingPane, UserDetails};

#[derive(Debug, Clone)]
pub struct UsersTabData {
    pub user_details: Vec<UserDetails>,
    pub edit_pane: EditingPane,
    pub add_user_button: button::State,
    pub edit_button_states: Vec<button::State>,
    pub delete_button_states: Vec<button::State>,
}

impl UsersTabData {
    pub fn new(user_details: Vec<UserDetails>) -> Self {
        let mut edit_button_states = Vec::new();
        let mut delete_button_states = Vec::new();

        for _ in 0..user_details.len() {
            edit_button_states.push(button::State::new());
            delete_button_states.push(button::State::new());
        }

        UsersTabData {
            user_details,
            edit_button_states,
            delete_button_states,
            add_user_button: button::State::new(),
            edit_pane: EditingPane::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EditUserDetails {
    pub name_value: String,
    pub name_input: text_input::State,
    pub shares_value: String,
    pub shares_input: text_input::State,
    pub error_message: String,
    pub confirm_button: button::State,
}

#[derive(Debug, Clone, Default)]
pub struct ConfirmDeletion {
    pub user_id: i32,
    pub cancel_button: button::State,
    pub delete_button: button::State,
    pub error_message: String,
}

pub fn render_users_tab<'a>(
    add_user_button: &'a mut button::State,
    user_details: &'a mut Vec<UserDetails>,
    edit_button_states: &'a mut Vec<button::State>,
    delete_button_states: &'a mut Vec<button::State>,
    edit_pane: &'a mut EditingPane,
) -> Column<'a, Message> {
    let mut row = Row::new().push(
        Column::new()
            .padding(20)
            .push(
                Container::new(
                    Button::new(
                        add_user_button,
                        Text::new("Add User").size(28).font(OPEN_SANS),
                    )
                    .style(style::Button::Confirm)
                    .on_press(Message::AddUserButtonPressed),
                )
                .padding(10),
            )
            .push(if user_details.len() > 0 {
                render_users_list(user_details, edit_button_states, delete_button_states)
            } else {
                Column::new()
            })
            .width(match edit_pane {
                EditingPane::Closed => iced::Length::Units(WINDOW_WIDTH - SIDEBAR_WIDTH),
                _ => iced::Length::Units(WINDOW_WIDTH - SIDEBAR_WIDTH - EDIT_PANE_WIDTH),
            }),
    );
    match edit_pane {
        EditingPane::Closed => {}
        EditingPane::AddingUser(edit_user_details) => {
            row = row.push(render_edit_pane(
                Column::new()
                    .push(
                        Text::new("Add new user")
                            .font(OPEN_SANS_BOLD)
                            .size(32)
                            .color(style::DARK_GREY),
                    )
                    .push(
                        Column::new()
                            .padding(20)
                            .push(
                                TextInput::new(
                                    &mut edit_user_details.name_input,
                                    "Username",
                                    &edit_user_details.name_value,
                                    Message::EditPaneUserNameInputChanged,
                                )
                                .padding(10)
                                .size(28)
                                .font(OPEN_SANS),
                            )
                            .push(
                                TextInput::new(
                                    &mut edit_user_details.shares_input,
                                    "Ammount of shares",
                                    &edit_user_details.shares_value,
                                    Message::EditPaneUserShareInputChanged,
                                )
                                .padding(10)
                                .size(28)
                                .font(OPEN_SANS),
                            ),
                    )
                    .push(
                        Text::new(edit_user_details.error_message.clone())
                            .size(28)
                            .font(OPEN_SANS)
                            .color(style::RED),
                    )
                    .push(
                        Button::new(
                            &mut edit_user_details.confirm_button,
                            Text::new("Confirm").size(28).font(OPEN_SANS),
                        )
                        .style(style::Button::Confirm)
                        .on_press(Message::EditPaneConfirmButtonClicked),
                    ),
            ))
        }
        EditingPane::EditingUser(_user_id, edit_user_details) => {
            row = row.push(render_edit_pane(
                Column::new()
                    .push(
                        Text::new(format!("Edit User"))
                            .font(OPEN_SANS_BOLD)
                            .size(32)
                            .color(style::DARK_GREY),
                    )
                    .push(
                        Column::new()
                            .padding(20)
                            .push(
                                TextInput::new(
                                    &mut edit_user_details.name_input,
                                    "Username",
                                    &edit_user_details.name_value,
                                    Message::EditPaneUserNameInputChanged,
                                )
                                .size(28)
                                .padding(10)
                                .font(OPEN_SANS),
                            )
                            .push(
                                TextInput::new(
                                    &mut edit_user_details.shares_input,
                                    "Ammount of shares",
                                    &edit_user_details.shares_value,
                                    Message::EditPaneUserShareInputChanged,
                                )
                                .size(28)
                                .padding(10)
                                .font(OPEN_SANS),
                            ),
                    )
                    .push(
                        Text::new(edit_user_details.error_message.clone())
                            .size(28)
                            .font(OPEN_SANS)
                            .color(style::RED),
                    )
                    .push(
                        Button::new(
                            &mut edit_user_details.confirm_button,
                            Text::new("Confirm").size(28).font(OPEN_SANS),
                        )
                        .style(style::Button::Confirm)
                        .on_press(Message::EditPaneConfirmButtonClicked),
                    ),
            ))
        }
        EditingPane::ConfirmingDeletion(ConfirmDeletion {
            user_id,
            delete_button,
            error_message,
            cancel_button,
        }) => {
            row = row.push(render_edit_pane(
                Column::new()
                    .push(
                        Text::new(format!(
                            "Are you sure you want to delete \"{}\"",
                            user_details
                                .clone()
                                .iter()
                                .find(|user| { user.id == *user_id })
                                .unwrap()
                                .name
                                .clone()
                        ))
                        .size(28)
                        .font(OPEN_SANS),
                    )
                    .push(
                        Row::new()
                            .push(
                                Button::new(
                                    cancel_button,
                                    Text::new("No").size(28).font(OPEN_SANS),
                                )
                                .on_press(Message::CloseEditPane)
                                .style(style::Button::Confirm),
                            )
                            .push(
                                Button::new(
                                    delete_button,
                                    Text::new("Yes").size(28).font(OPEN_SANS),
                                )
                                .on_press(Message::EditPaneConfirmButtonClicked)
                                .style(style::Button::Destructive),
                            ),
                    )
                    .push(
                        Text::new(error_message.clone())
                            .size(28)
                            .font(OPEN_SANS)
                            .color(style::RED),
                    ),
            ))
        }
        _ => {}
    }
    Column::new().push(row)
}

pub fn render_users_list<'a>(
    user_details: &'a Vec<UserDetails>,
    edit_button_states: &'a mut Vec<button::State>,
    delete_button_states: &'a mut Vec<button::State>,
) -> Column<'a, Message> {
    let mut col = Column::new().padding(10).push(
        Row::new()
            .push(
                Container::new(
                    Text::new("Users")
                        .size(28)
                        .font(OPEN_SANS_BOLD)
                        .width(iced::Length::Units(200)),
                )
                .width(iced::Length::Units(200))
                .padding(10),
            )
            .push(
                Container::new(
                    Text::new("Shares (K)")
                        .size(28)
                        .font(OPEN_SANS_BOLD)
                        .width(iced::Length::Units(200))
                        .horizontal_alignment(iced::HorizontalAlignment::Right),
                )
                .width(iced::Length::Units(200))
                .padding(10),
            ),
    );

    let mut edit_buttons = Vec::new();

    for (i, state) in edit_button_states.iter_mut().enumerate() {
        edit_buttons.push(
            Button::new(state, Text::new("edit").font(OPEN_SANS))
                .style(style::Button::Icon)
                .on_press(Message::EditUserButtonPressed(user_details[i].id)),
        );
    }

    let mut delete_buttons = Vec::new();

    for (i, state) in delete_button_states.iter_mut().enumerate() {
        delete_buttons.push(
            Button::new(state, Text::new("delete").font(OPEN_SANS))
                .style(style::Button::IconDestructive)
                .on_press(Message::DeleteUserButtonPressed(user_details[i].id)),
        );
    }

    for (i, user) in user_details.iter().enumerate() {
        col = col.push(
            Container::new(
                Row::new()
                    .push(
                        Container::new(
                            Text::new(user.name.clone())
                                .width(iced::Length::Units(200))
                                .size(28)
                                .font(OPEN_SANS),
                        )
                        .padding(10),
                    )
                    .push(
                        Container::new(
                            Text::new(format_decimal(user.contribution))
                                .width(iced::Length::Units(200))
                                .horizontal_alignment(iced::HorizontalAlignment::Right)
                                .size(28)
                                .font(OPEN_SANS),
                        )
                        .width(iced::Length::Units(200))
                        .padding(10),
                    )
                    .push(edit_buttons.remove(0))
                    .push(delete_buttons.remove(0)),
            )
            .style(if i % 2 == 0 {
                style::TableRow::Lighter
            } else {
                style::TableRow::Darker
            }),
        )
    }

    col
}
