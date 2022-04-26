use iced::{button, Button, Column, Container, Text};

use crate::{
    family_banking::Message,
    style::{self, OPEN_SANS},
    EDIT_PANE_WIDTH, SIDEBAR_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};

use self::home_tab::HomeTabData;

pub mod debts_tab;
pub mod home_tab;
pub mod users_tab;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Home,
    Users,
    Debts,
}

impl Default for Tab {
    fn default() -> Tab {
        Tab::Home
    }
}

#[derive(Debug)]
pub enum TabData {
    Home(rusqlite::Result<HomeTabData>),
    Users(rusqlite::Result<users_tab::UsersTabData>),
    Debts(rusqlite::Result<debts_tab::DebtsTabData>),
}

impl Default for TabData {
    fn default() -> Self {
        Self::Home(Ok(HomeTabData::default()))
    }
}

#[derive(Debug, Clone)]
pub enum EditingPane {
    Closed,
    AddingUser(users_tab::EditUserDetails),
    EditingUser(i32, users_tab::EditUserDetails),
    ConfirmingDeletion(users_tab::ConfirmDeletion),
    AddingDebt(debts_tab::AddDebt),
    RepayingDebt(debts_tab::RepayDebt),
}

impl Default for EditingPane {
    fn default() -> Self {
        Self::Closed
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserDetails {
    pub id: i32,
    pub name: String,
    pub contribution: f64,
    pub percent: f64,
    pub loan: f64,
    pub interest: f64,
}

#[derive(Debug, Default)]
pub struct MainView {
    pub current_tab: Tab,
    pub tab_data: TabData,
    pub home_button: button::State,
    pub users_button: button::State,
    pub debts_button: button::State,
}

fn render_main_view_error(err: &rusqlite::Error) -> Column<Message> {
    Column::new().push(
        Text::new(format!("Error while fetching data: {}", err.to_string()))
            .size(28)
            .font(OPEN_SANS)
            .color(style::RED),
    )
}

pub fn render_tab_buttons<'a>(
    home_button: &'a mut button::State,
    users_button: &'a mut button::State,
    debts_button: &'a mut button::State,
    current_tab: Tab,
) -> Column<'a, Message> {
    #[inline]
    fn change_tab_button<'a>(
        state: &'a mut iced::button::State,
        text: &str,
        tab: Tab,
        current_tab: Tab,
    ) -> button::Button<'a, Message> {
        Button::new(
            state,
            Text::new(text)
                .size(22)
                .horizontal_alignment(iced::HorizontalAlignment::Center)
                .font(OPEN_SANS),
        )
        .on_press(Message::TabButtonPressed(tab))
        .width(iced::Length::Units(SIDEBAR_WIDTH))
        .style(if current_tab == tab {
            style::SidebarButton::Selected
        } else {
            style::SidebarButton::Deselected
        })
    }

    Column::new()
        // .padding(15)
        .push(change_tab_button(
            home_button,
            "Home",
            Tab::Home,
            current_tab,
        ))
        .push(change_tab_button(
            users_button,
            "Users",
            Tab::Users,
            current_tab,
        ))
        .push(change_tab_button(
            debts_button,
            "Debts",
            Tab::Debts,
            current_tab,
        ))
}

fn render_edit_pane(contents: Column<Message>) -> Container<Message> {
    Container::new(contents.align_items(iced::Align::Center))
        .padding(10)
        .height(iced::Length::Units(WINDOW_HEIGHT))
        .width(iced::Length::Units(EDIT_PANE_WIDTH))
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
        .style(style::EditPane)
}

pub fn render_main_view<'a>(tab_data: &'a mut TabData) -> Container<'a, Message> {
    Container::new(Column::new().push(match tab_data {
        TabData::Home(htd_result) => match htd_result {
            Ok(HomeTabData {
                total_cash,
                total_shares,
                total_loans,
                total_debt,
                profit,
                user_details,
            }) => home_tab::render_home_tab(
                *total_cash,
                *total_debt,
                *total_shares,
                *profit,
                user_details.clone(),
            ),
            Err(err) => render_main_view_error(err),
        },
        TabData::Users(users_result) => match users_result {
            Ok(users_tab::UsersTabData {
                user_details,
                edit_pane,
                add_user_button,
                edit_button_states,
                delete_button_states,
            }) => users_tab::render_users_tab(
                add_user_button,
                user_details,
                edit_button_states,
                delete_button_states,
                edit_pane,
            ),
            Err(err) => render_main_view_error(err),
        },
        TabData::Debts(debts_result) => match debts_result {
            Ok(debts_tab::DebtsTabData {
                user_details,
                edit_pane,
                add_button_states,
                repay_button_states,
            }) => debts_tab::render_debts_tab(
                user_details,
                add_button_states,
                repay_button_states,
                edit_pane,
            ),
            Err(err) => render_main_view_error(err),
        },
    }))
    .style(style::TabContents)
    .width(iced::Length::Units(WINDOW_WIDTH - SIDEBAR_WIDTH))
    .height(iced::Length::Units(WINDOW_HEIGHT))
}
