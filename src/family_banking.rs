use std::str::FromStr;

use iced::{Application, Color, Column, Element, Row};

use crate::{
    db_operations,
    login_view::{render_login_view, render_new_password_view, LoginView},
    main_view::{
        self,
        debts_tab::{AddDebt, RepayDebt},
        render_tab_buttons,
        users_tab::ConfirmDeletion,
        users_tab::EditUserDetails,
        EditingPane, MainView, Tab, TabData,
    },
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

pub struct FamilyBanking {
    pub db_connection: rusqlite::Connection,
    pub status: Status,
    pub admin_password: Option<String>,
}

#[derive(Debug)]
pub enum Status {
    LoggedIn(MainView),
    NotLoggedIn(LoginView),
}

impl Default for Status {
    fn default() -> Self {
        Status::NotLoggedIn(LoginView::default())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TabButtonPressed(Tab),
    PasswordInput1Changed(String),
    PasswordInput2Changed(String),
    LoginButtonPressed,
    NewPasswordButtonPressed,
    AddUserButtonPressed,
    EditUserButtonPressed(i32),
    DeleteUserButtonPressed(i32),
    EditPaneUserNameInputChanged(String),
    EditPaneUserShareInputChanged(String),
    EditPaneConfirmButtonClicked,
    AddDebtButtonPressed(i32),
    RepayDebtButtonPressed(i32),
    EditPaneDebtInputChanged(String),
    EditPaneRepaymentInputChanged(String),
    EditPaneInterestInputChanged(String),
    CloseEditPane,
}

pub struct Flags {
    pub admin_password: Option<String>,
    pub db_connection: rusqlite::Connection,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            admin_password: None,
            db_connection: rusqlite::Connection::open("./db.sqlite3").unwrap(),
        }
    }
}

impl Application for FamilyBanking {
    type Message = Message;

    type Executor = iced::executor::Default;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Message>) {
        (
            FamilyBanking {
                db_connection: flags.db_connection,
                admin_password: flags.admin_password,
                status: Status::default(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Family Banking")
    }

    fn update(
        &mut self,
        message: Message,
        clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Message> {
        match message {
            Message::TabButtonPressed(tab) => match &mut self.status {
                Status::LoggedIn(_) => {
                    let tab_data = match tab {
                        Tab::Home => {
                            TabData::Home(db_operations::home_tab_data(&self.db_connection))
                        }
                        Tab::Users => {
                            TabData::Users(db_operations::users_tab_data(&self.db_connection))
                        }
                        Tab::Debts => {
                            TabData::Debts(db_operations::debts_tab_data(&self.db_connection))
                        }
                    };
                    self.status = Status::LoggedIn(MainView {
                        current_tab: tab,
                        tab_data: tab_data,
                        ..MainView::default()
                    })
                }
                _ => {}.clone(),
            },
            Message::PasswordInput1Changed(text) => {
                match &mut self.status {
                    Status::NotLoggedIn(view) => {
                        self.status = Status::NotLoggedIn(LoginView {
                            password_input_1_value: text,
                            ..view.clone()
                        })
                    }
                    _ => {}
                };
            }
            Message::PasswordInput2Changed(text) => {
                match &mut self.status {
                    Status::NotLoggedIn(view) => {
                        self.status = Status::NotLoggedIn(LoginView {
                            password_input_2_value: text,
                            ..view.clone()
                        })
                    }
                    _ => {}
                };
            }
            Message::LoginButtonPressed => match &self.admin_password {
                Some(hash) => match &mut self.status {
                    Status::NotLoggedIn(view) => {
                        let matches = match argon2::verify_encoded(
                            &hash,
                            view.password_input_1_value.as_bytes(),
                        ) {
                            Ok(boolean) => boolean,
                            Err(_) => false,
                        };

                        if matches {
                            self.status = Status::LoggedIn(MainView {
                                tab_data: TabData::Home(db_operations::home_tab_data(
                                    &self.db_connection,
                                )),
                                ..MainView::default()
                            });
                        } else {
                            self.status = Status::NotLoggedIn(LoginView {
                                login_error_message: String::from("Incorrect password"),
                                ..LoginView::default()
                            });
                        }
                    }
                    _ => {}
                },
                None => {}
            },
            Message::NewPasswordButtonPressed => match &mut self.status {
                Status::NotLoggedIn(login_view_data) => {
                    if login_view_data.password_input_1_value.len() < 6
                        || login_view_data.password_input_2_value.len() < 6
                    {
                        login_view_data.login_error_message =
                            String::from("Password needs to be at least 6 characters long");
                    } else if login_view_data.password_input_1_value
                        != login_view_data.password_input_2_value
                    {
                        login_view_data.login_error_message = String::from("Passwords do not match")
                    } else {
                        db_operations::store_password(
                            &self.db_connection,
                            login_view_data.password_input_1_value.clone(),
                        );

                        self.status = Status::LoggedIn(MainView {
                            tab_data: TabData::Home(db_operations::home_tab_data(
                                &self.db_connection,
                            )),
                            ..MainView::default()
                        })
                    }
                }
                _ => {}
            },
            Message::EditUserButtonPressed(user_id) => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Users(utd_result) => match utd_result {
                        Ok(users_tab_data) => {
                            let user = users_tab_data
                                .user_details
                                .clone()
                                .iter()
                                .find(|u| u.id == user_id)
                                .unwrap()
                                .clone();

                            users_tab_data.edit_pane = EditingPane::EditingUser(
                                user_id,
                                EditUserDetails {
                                    name_value: user.name.clone(),
                                    shares_value: user.contribution.to_string(),
                                    ..EditUserDetails::default()
                                },
                            )
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Message::DeleteUserButtonPressed(user_id) => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Users(utd_result) => match utd_result {
                        Ok(users_tab_data) => {
                            users_tab_data.edit_pane =
                                EditingPane::ConfirmingDeletion(ConfirmDeletion {
                                    user_id,
                                    ..Default::default()
                                })
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Message::AddUserButtonPressed => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Users(utd_result) => match utd_result {
                        Ok(users_tab_data) => {
                            users_tab_data.edit_pane =
                                EditingPane::AddingUser(EditUserDetails::default())
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Message::EditPaneUserNameInputChanged(value) => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Users(utd_result) => match utd_result {
                        Ok(users_tab_data) => match &mut users_tab_data.edit_pane {
                            EditingPane::AddingUser(edit_user_details) => {
                                edit_user_details.name_value = value
                            }
                            EditingPane::EditingUser(_user_id, edit_user_details) => {
                                edit_user_details.name_value = value
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Message::EditPaneUserShareInputChanged(value) => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Users(utd_result) => match utd_result {
                        Ok(users_tab_data) => match &mut users_tab_data.edit_pane {
                            EditingPane::AddingUser(edit_user_details) => {
                                edit_user_details.shares_value = value
                            }
                            EditingPane::EditingUser(_user_id, edit_user_details) => {
                                edit_user_details.shares_value = value
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Message::EditPaneConfirmButtonClicked => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Users(utd_result) => match utd_result {
                        Ok(users_tab_data) => match &mut users_tab_data.edit_pane {
                            EditingPane::AddingUser(edit_user_details) => {
                                if edit_user_details.name_value == "".to_string() {
                                    edit_user_details.error_message =
                                        "Enter valid username".to_string()
                                } else if let Ok(shares) = edit_user_details.shares_value.parse()
                                    as Result<f64, <f64 as FromStr>::Err>
                                {
                                    match db_operations::store_new_user(
                                        &self.db_connection,
                                        edit_user_details.name_value.clone(),
                                        shares,
                                    ) {
                                        Ok(_) => {
                                            self.status = Status::LoggedIn(MainView {
                                                current_tab: Tab::Users,
                                                tab_data: TabData::Users(
                                                    db_operations::users_tab_data(
                                                        &self.db_connection,
                                                    ),
                                                ),
                                                ..MainView::default()
                                            })
                                        }
                                        Err(err) => {
                                            edit_user_details.error_message = err.to_string()
                                        }
                                    }
                                } else {
                                    edit_user_details.error_message =
                                        "Enter valid number".to_string()
                                }
                            }
                            EditingPane::EditingUser(user_id, edit_user_details) => {
                                if edit_user_details.name_value == "".to_string() {
                                    edit_user_details.error_message =
                                        "Enter valid username".to_string()
                                } else if let Ok(shares) = edit_user_details.shares_value.parse()
                                    as Result<f64, <f64 as FromStr>::Err>
                                {
                                    match db_operations::edit_user(
                                        &self.db_connection,
                                        *user_id,
                                        edit_user_details.name_value.clone(),
                                        shares,
                                    ) {
                                        Ok(_) => {
                                            self.status = Status::LoggedIn(MainView {
                                                current_tab: Tab::Users,
                                                tab_data: TabData::Users(
                                                    db_operations::users_tab_data(
                                                        &self.db_connection,
                                                    ),
                                                ),
                                                ..MainView::default()
                                            })
                                        }
                                        Err(err) => {
                                            edit_user_details.error_message = err.to_string()
                                        }
                                    }
                                } else {
                                    edit_user_details.error_message =
                                        "Enter valid number".to_string()
                                }
                            }
                            EditingPane::ConfirmingDeletion(confirm_deletion) => {
                                match db_operations::delete_user(
                                    &self.db_connection,
                                    confirm_deletion.user_id,
                                ) {
                                    Ok(_) => {
                                        self.status = Status::LoggedIn(MainView {
                                            current_tab: Tab::Users,
                                            tab_data: TabData::Users(
                                                db_operations::users_tab_data(&self.db_connection),
                                            ),
                                            ..MainView::default()
                                        })
                                    }
                                    Err(err) => confirm_deletion.error_message = err.to_string(),
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    },

                    TabData::Debts(dtd_result) => match dtd_result {
                        Ok(debts_tab_data) => match &mut debts_tab_data.edit_pane {
                            EditingPane::AddingDebt(add_debt) => {
                                if let Ok(loan) = add_debt.debt_value.parse()
                                    as Result<f64, <f64 as FromStr>::Err>
                                {
                                    if let Ok(interest) = add_debt.interest_value.parse()
                                        as Result<f64, <f64 as FromStr>::Err>
                                    {
                                        if loan < 0.0 {
                                            add_debt.error_message =
                                                "Loan ammount cannot be negative".to_string()
                                        } else if interest < 0.0 {
                                            add_debt.error_message =
                                                "Interest cannot be negative".to_string()
                                        } else {
                                            match db_operations::borrow_debt(
                                                &self.db_connection,
                                                add_debt.user_id,
                                                loan,
                                                interest,
                                            ) {
                                                Ok(_) => {
                                                    self.status = Status::LoggedIn(MainView {
                                                        current_tab: Tab::Debts,
                                                        tab_data: TabData::Debts(
                                                            db_operations::debts_tab_data(
                                                                &self.db_connection,
                                                            ),
                                                        ),
                                                        ..MainView::default()
                                                    })
                                                }
                                                Err(err) => {
                                                    add_debt.error_message = err.to_string()
                                                }
                                            }
                                        }
                                    } else {
                                        add_debt.error_message = "Invalid interest".to_string()
                                    }
                                } else {
                                    add_debt.error_message = "Invalid loan ammount".to_string()
                                }
                            }
                            EditingPane::RepayingDebt(repay_debt) => {
                                if let Ok(repayment) = repay_debt.repayment_value.parse()
                                    as Result<f64, <f64 as FromStr>::Err>
                                {
                                    if repayment < 0.0 {
                                        repay_debt.error_message =
                                            "Repayment ammount cannot be negative".to_string()
                                    } else {
                                        let user = debts_tab_data
                                            .user_details
                                            .iter()
                                            .find(|u| u.id == repay_debt.user_id)
                                            .unwrap();

                                        if user.loan + user.interest >= repayment {
                                            let (loan_repayment, interest_repayment) =
                                                if user.loan > repayment {
                                                    (repayment, 0.0)
                                                } else {
                                                    (user.loan, repayment - user.loan)
                                                };

                                            match db_operations::repay_debt(
                                                &self.db_connection,
                                                repay_debt.user_id,
                                                loan_repayment,
                                                interest_repayment,
                                            ) {
                                                Ok(_) => {
                                                    self.status = Status::LoggedIn(MainView {
                                                        current_tab: Tab::Debts,
                                                        tab_data: TabData::Debts(
                                                            db_operations::debts_tab_data(
                                                                &self.db_connection,
                                                            ),
                                                        ),
                                                        ..MainView::default()
                                                    })
                                                }
                                                Err(err) => {
                                                    repay_debt.error_message = err.to_string()
                                                }
                                            }
                                        } else {
                                            repay_debt.error_message =
                                                "Repayment is higher than debt".to_string()
                                        }
                                    }
                                } else {
                                    repay_debt.error_message = "Enter valid number".to_string()
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Message::AddDebtButtonPressed(user_id) => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Debts(dtd_result) => match dtd_result {
                        Ok(debts_tab_data) => {
                            debts_tab_data.edit_pane = EditingPane::AddingDebt(AddDebt {
                                user_id,
                                ..Default::default()
                            })
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Message::RepayDebtButtonPressed(user_id) => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Debts(dtd_result) => match dtd_result {
                        Ok(debts_tab_data) => {
                            debts_tab_data.edit_pane = EditingPane::RepayingDebt(RepayDebt {
                                user_id,
                                ..Default::default()
                            })
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Message::EditPaneDebtInputChanged(value) => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Debts(dtd_result) => match dtd_result {
                        Ok(debts_tab_data) => match &mut debts_tab_data.edit_pane {
                            EditingPane::AddingDebt(add_debt) => add_debt.debt_value = value,
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Message::EditPaneInterestInputChanged(value) => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Debts(dtd_result) => match dtd_result {
                        Ok(debts_tab_data) => match &mut debts_tab_data.edit_pane {
                            EditingPane::AddingDebt(add_debt) => add_debt.interest_value = value,
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Message::EditPaneRepaymentInputChanged(value) => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Debts(dtd_result) => match dtd_result {
                        Ok(debts_tab_data) => match &mut debts_tab_data.edit_pane {
                            EditingPane::RepayingDebt(repay_debt) => {
                                repay_debt.repayment_value = value
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            Message::CloseEditPane => match &mut self.status {
                Status::LoggedIn(main_view) => match &mut main_view.tab_data {
                    TabData::Users(utd_result) => match utd_result {
                        Ok(utd) => utd.edit_pane = EditingPane::Closed,
                        _ => {}
                    },
                    TabData::Debts(dtd_result) => match dtd_result {
                        Ok(dtd) => dtd.edit_pane = EditingPane::Closed,
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
        };
        iced::Command::none()
    }

    fn view<'a>(&mut self) -> Element<Message> {
        Column::new()
            .width(iced::Length::Units(WINDOW_WIDTH))
            .height(iced::Length::Units(WINDOW_HEIGHT))
            .align_items(iced::Align::Center)
            .push(match &mut self.status {
                Status::NotLoggedIn(login_view_data) => match &self.admin_password {
                    None => render_new_password_view(login_view_data),
                    Some(_) => render_login_view(login_view_data),
                },
                Status::LoggedIn(main_view_data) => Column::new().width(iced::Length::Fill).push(
                    Row::new()
                        .width(iced::Length::Fill)
                        .push(render_tab_buttons(
                            &mut main_view_data.home_button,
                            &mut main_view_data.users_button,
                            &mut main_view_data.debts_button,
                            main_view_data.current_tab,
                        ))
                        .push(main_view::render_main_view(&mut main_view_data.tab_data)),
                ),
            })
            .into()
    }

    fn background_color(&self) -> Color {
        Color::WHITE
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }

    fn mode(&self) -> iced::window::Mode {
        iced::window::Mode::Windowed
    }

    fn should_exit(&self) -> bool {
        false
    }
}
