extern crate argon2;

use iced::{button, Button, Color, Column, Element, Font, Row, Sandbox, Settings, Text};
use rusqlite::{params, Connection, Result, Statement};

const HASH_SALT: &[u8; 17] = b"5aP3v*4!1bN<x4i&3";

fn main() {
    let conn = match Connection::open("./db.sqlite3") {
        Ok(conn) => {
            println!("Auto-commit: {}", conn.is_autocommit());
            conn
        }
        Err(e) => {
            panic!("{}", e)
        }
    };

    let mut password_stmt = conn.prepare("SELECT passwordHash FROM admin");

    let password = match password_stmt {
        Ok(mut stmt) => {
            let mut rows = stmt.query([]).unwrap();

            let password_hash = if let Some(row) = rows.next().unwrap() {
                let hash: String = row.get(0).unwrap();
                println!("Password hash: {}", hash);
                Option::Some(hash)
            } else {
                println!("No password hash found");
                Option::None
            };

            password_hash
        }
        _ => {
            println!("Attempting to initialize database tables...");
            initialize_db_tables(&conn).unwrap();
            Option::None
        }
    };

    FamilyBanking::run(Settings::default());
}

fn initialize_db_tables(conn: &Connection) -> Result<usize> {
    let mut rows_changed = conn.execute(
        "
        CREATE TABLE admin (
            id              INTEGER PRIMARY KEY,
            passwordHash    TEXT NOT NULL
        );
",
        [],
    )?;
    rows_changed += conn.execute(
        "
        CREATE TABLE member (
            memberId    INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            share       DECIMAL
        );
",
        [],
    )?;
    rows_changed += conn.execute(
        "
        CREATE TABLE debt (
            debtId      INTEGER PRIMARY KEY,
            memberId    INTEGER NOT NULL,
            ammount     DECIMAL NOT NULL,
            interest    DECIMAL NOT NULL,
            paid        BOOLEAN,
            dateTaken   DATE,
            datePaid    DATE,
                FOREIGN KEY (memberId) REFERENCES member (memberId)
        );
",
        [],
    )?;
    Ok(rows_changed
        + conn.execute(
            "
        CREATE TABLE deposit (
            depositId   INTEGER PRIMARY KEY,
            memberId    INTEGER NOT NULL, 
            ammount     DECIMAL NOT NULL,
            date        DATE,
                FOREIGN KEY (memberId) REFERENCES member (memberId)
        );",
            [],
        )?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CurrentTab {
    Login,
    Home,
    Users,
    Debts,
    History,
    Settings,
}

impl Default for CurrentTab {
    fn default() -> CurrentTab {
        CurrentTab::Login
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TabButtonPressed(CurrentTab),
}

// Font
const OPEN_SANS: Font = Font::External {
    name: "Open Sans",
    bytes: include_bytes!("../fonts/OpenSans-Regular.ttf"),
};

const ACCENT: Color = Color::from_rgb(0.0, 0.76, 0.04);

#[derive(Default)]
struct FamilyBanking {
    current_tab: CurrentTab,
    home_button: button::State,
    users_button: button::State,
    debts_button: button::State,
    history_button: button::State,
    settings_button: button::State,
}

impl Sandbox for FamilyBanking {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Family Banking")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TabButtonPressed(tab) => self.current_tab = tab,
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            // .padding(20)
            .width(iced::Length::Fill)
            .push(
                Row::new().width(iced::Length::Fill).padding(15).push(
                    Text::new("Family Banking")
                        .size(42)
                        .font(OPEN_SANS)
                        // .width(iced::Length::Fill)
                        .horizontal_alignment(iced::HorizontalAlignment::Center),
                ),
            )
            .push(
                Row::new().push(self.tab_buttons()).push(
                    Column::new()
                        .padding(50)
                        .push(Text::new("Some Tab Content").font(OPEN_SANS)),
                ),
            )
            .into()
    }

    fn background_color(&self) -> Color {
        Color::WHITE
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn run(settings: Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
}

impl FamilyBanking {
    fn tab_buttons(&mut self) -> Column<Message> {
        #[inline]
        fn change_tab_button<'a>(
            state: &'a mut iced::button::State,
            text: &str,
            tab: CurrentTab,
            current_tab: CurrentTab,
        ) -> button::Button<'a, Message> {
            Button::new(
                state,
                Text::new(text)
                    .horizontal_alignment(iced::HorizontalAlignment::Center)
                    .font(OPEN_SANS),
            )
            .on_press(Message::TabButtonPressed(tab))
            .width(iced::Length::Units(150))
            .style(if current_tab == tab {
                style::Button::FilterSelected
            } else {
                style::Button::FilterActive
            })
        }

        Column::new()
            // .padding(15)
            .push(change_tab_button(
                &mut self.home_button,
                "Home",
                CurrentTab::Home,
                self.current_tab,
            ))
            .push(change_tab_button(
                &mut self.users_button,
                "Users",
                CurrentTab::Users,
                self.current_tab,
            ))
            .push(change_tab_button(
                &mut self.debts_button,
                "Debts",
                CurrentTab::Debts,
                self.current_tab,
            ))
            .push(change_tab_button(
                &mut self.history_button,
                "History",
                CurrentTab::History,
                self.current_tab,
            ))
            .push(change_tab_button(
                &mut self.settings_button,
                "Settings",
                CurrentTab::Settings,
                self.current_tab,
            ))
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    use crate::ACCENT;

    pub enum Button {
        FilterActive,
        FilterSelected,
        Icon,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::FilterActive => button::Style::default(),
                Button::FilterSelected => button::Style {
                    background: Some(Background::Color(ACCENT)),
                    border_radius: 0.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                },
                Button::Icon => button::Style {
                    text_color: Color::from_rgb(0.5, 0.5, 0.5),
                    ..button::Style::default()
                },
                Button::Destructive => button::Style {
                    background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                    border_radius: 0.0,
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
                    Button::Icon => ACCENT,
                    Button::FilterActive => ACCENT,
                    _ => active.text_color,
                },
                shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                ..active
            }
        }
    }

    pub struct BorderedContainer {}

    impl iced::container::StyleSheet for BorderedContainer {
        fn style(&self) -> iced::container::Style {
            iced::container::Style {
                ..iced::container::Style::default()
            }
        }
    }
}
