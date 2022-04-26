use crate::{
    family_banking::Message,
    format_decimal, style,
    style::{ACCENT_COLOR, OPEN_SANS, OPEN_SANS_BOLD},
    EDIT_PANE_WIDTH, SIDEBAR_WIDTH, WINDOW_WIDTH,
};

use iced::{button, text_input, Button, Column, Container, Row, Text, TextInput};

use super::{render_edit_pane, EditingPane, UserDetails};

#[derive(Debug, Clone)]
pub struct DebtsTabData {
    pub user_details: Vec<UserDetails>,
    pub edit_pane: EditingPane,
    pub add_button_states: Vec<button::State>,
    pub repay_button_states: Vec<button::State>,
}

impl DebtsTabData {
    pub fn new(user_details: Vec<UserDetails>) -> Self {
        let mut add_button_states = Vec::new();
        let mut repay_button_states = Vec::new();

        for _ in 0..user_details.len() {
            add_button_states.push(button::State::new());
            repay_button_states.push(button::State::new());
        }

        DebtsTabData {
            user_details,
            add_button_states,
            repay_button_states,
            edit_pane: EditingPane::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AddDebt {
    pub user_id: i32,
    pub debt_value: String,
    pub debt_input: text_input::State,
    pub interest_value: String,
    pub interest_input: text_input::State,
    pub error_message: String,
    pub confirm_button: button::State,
}

#[derive(Debug, Clone, Default)]
pub struct RepayDebt {
    pub user_id: i32,
    pub repayment_value: String,
    pub repayment_input: text_input::State,
    pub error_message: String,
    pub confirm_button: button::State,
}

pub fn render_debts_tab<'a>(
    user_details: &'a mut Vec<UserDetails>,
    add_button_states: &'a mut Vec<button::State>,
    repay_button_states: &'a mut Vec<button::State>,
    edit_pane: &'a mut EditingPane,
) -> Column<'a, Message> {
    let mut row =
        Row::new().push(
            Column::new()
                .padding(20)
                .push(
                    Container::new(
                        Row::new()
                            .push(Text::new("Total Debt: ").size(32).font(OPEN_SANS))
                            .push(
                                Text::new(format!(
                                    "K{}",
                                    format_decimal(user_details.iter().fold(0.0, |acc, user| {
                                        acc + user.interest + user.loan
                                    }))
                                ))
                                .color(ACCENT_COLOR)
                                .size(32)
                                .font(OPEN_SANS_BOLD),
                            ),
                    )
                    .padding(10),
                )
                .push(if user_details.len() > 0 {
                    render_debts_list(user_details, add_button_states, repay_button_states)
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
        EditingPane::AddingDebt(add_debt) => {
            row = row.push(render_edit_pane(
                Column::new()
                    .push(
                        Text::new(format!(
                            "Lend money to {}",
                            user_details
                                .iter()
                                .find(|u| { u.id == add_debt.user_id })
                                .unwrap()
                                .name
                        ))
                        .font(OPEN_SANS_BOLD)
                        .size(32)
                        .color(style::DARK_GREY),
                    )
                    .push(
                        Column::new()
                            .padding(20)
                            .push(
                                TextInput::new(
                                    &mut add_debt.debt_input,
                                    "Loan ammount",
                                    &add_debt.debt_value,
                                    Message::EditPaneDebtInputChanged,
                                )
                                .padding(10)
                                .size(28)
                                .font(OPEN_SANS),
                            )
                            .push(
                                TextInput::new(
                                    &mut add_debt.interest_input,
                                    "Interest",
                                    &add_debt.interest_value,
                                    Message::EditPaneInterestInputChanged,
                                )
                                .padding(10)
                                .size(28)
                                .font(OPEN_SANS),
                            ),
                    )
                    .push(
                        Text::new(add_debt.error_message.clone())
                            .size(28)
                            .font(OPEN_SANS)
                            .color(style::RED),
                    )
                    .push(
                        Button::new(
                            &mut add_debt.confirm_button,
                            Text::new("Confirm").size(28).font(OPEN_SANS),
                        )
                        .style(style::Button::Confirm)
                        .on_press(Message::EditPaneConfirmButtonClicked),
                    ),
            ))
        }
        EditingPane::RepayingDebt(repay_debt) => {
            row = row.push(render_edit_pane(
                Column::new()
                    .push(
                        Text::new(format!(
                            "Repay debt for {}",
                            user_details
                                .iter()
                                .find(|u| { u.id == repay_debt.user_id })
                                .unwrap()
                                .name
                        ))
                        .font(OPEN_SANS_BOLD)
                        .size(32)
                        .color(style::DARK_GREY),
                    )
                    .push(
                        Column::new().padding(20).push(
                            TextInput::new(
                                &mut repay_debt.repayment_input,
                                "Repayment",
                                &repay_debt.repayment_value,
                                Message::EditPaneRepaymentInputChanged,
                            )
                            .padding(10)
                            .size(28)
                            .font(OPEN_SANS),
                        ),
                    )
                    .push(
                        Text::new(repay_debt.error_message.clone())
                            .size(28)
                            .font(OPEN_SANS)
                            .color(style::RED),
                    )
                    .push(
                        Button::new(
                            &mut repay_debt.confirm_button,
                            Text::new("Confirm").size(28).font(OPEN_SANS),
                        )
                        .style(style::Button::Confirm)
                        .on_press(Message::EditPaneConfirmButtonClicked),
                    ),
            ))
        }
        _ => {}
    }
    Column::new().push(row)
}

pub fn render_debts_list<'a>(
    user_details: &'a Vec<UserDetails>,
    add_button_states: &'a mut Vec<button::State>,
    repay_button_states: &'a mut Vec<button::State>,
) -> Column<'a, Message> {
    let mut col = Column::new().padding(10).push(
        Row::new()
            .push(
                Container::new(
                    Text::new("User")
                        .size(28)
                        .width(iced::Length::Fill)
                        .font(OPEN_SANS_BOLD),
                )
                .padding(10)
                .width(iced::Length::Units(200)),
            )
            .push(
                Container::new(
                    Text::new("Debt (K)")
                        .size(28)
                        .width(iced::Length::Fill)
                        .horizontal_alignment(iced::HorizontalAlignment::Right)
                        .font(OPEN_SANS_BOLD),
                )
                .padding(10)
                .width(iced::Length::Units(200)),
            ),
    );

    let mut add_buttons = Vec::new();

    for (i, state) in add_button_states.iter_mut().enumerate() {
        add_buttons.push(
            Button::new(state, Text::new("Lend").font(OPEN_SANS))
                .style(style::Button::IconDestructive)
                .on_press(Message::AddDebtButtonPressed(user_details[i].id)),
        );
    }

    let mut repay_buttons = Vec::new();

    for (i, state) in repay_button_states.iter_mut().enumerate() {
        repay_buttons.push(
            Button::new(state, Text::new("Repay").font(OPEN_SANS))
                .style(style::Button::Icon)
                .on_press(Message::RepayDebtButtonPressed(user_details[i].id)),
        );
    }

    for (i, user) in user_details.iter().enumerate() {
        col = col.push(
            Container::new(
                Row::new()
                    .push(
                        Row::new()
                            .push(
                                Container::new(
                                    Text::new(user.name.clone())
                                        .width(iced::Length::Fill)
                                        .size(28)
                                        .font(OPEN_SANS),
                                )
                                .width(iced::Length::Units(200))
                                .padding(10),
                            )
                            .push(
                                Container::new(
                                    Text::new(format_decimal(user.interest + user.loan))
                                        .horizontal_alignment(iced::HorizontalAlignment::Right)
                                        .width(iced::Length::Fill)
                                        .size(28)
                                        .font(OPEN_SANS),
                                )
                                .width(iced::Length::Units(200))
                                .padding(10),
                            ),
                    )
                    .push(add_buttons.remove(0))
                    .push(repay_buttons.remove(0)),
            )
            .style(if i % 2 == 0 {
                style::TableRow::Lighter
            } else {
                style::TableRow::Darker
            }),
        );
    }

    col
}
