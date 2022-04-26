extern crate argon2;

use family_banking::{FamilyBanking, Flags};
use iced::{window, Application, Settings};
use rusqlite;
mod db_operations;
mod family_banking;
mod login_view;
mod main_view;
mod style;

const HASH_SALT: &[u8; 17] = b"5aP3v*4!1bN<x4i&3";

const WINDOW_WIDTH: u16 = 1100;
const WINDOW_HEIGHT: u16 = 600;
const SIDEBAR_WIDTH: u16 = 200;
const EDIT_PANE_WIDTH: u16 = 300;

fn main() -> Result<(), iced::Error> {
    let conn = rusqlite::Connection::open("./data.store").unwrap();

    let password = db_operations::get_password(&conn);

    FamilyBanking::run(Settings {
        flags: Flags {
            admin_password: password,
            db_connection: conn,
        },
        window: window::Settings {
            size: (WINDOW_WIDTH.into(), WINDOW_HEIGHT.into()),
            resizable: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

/// Converts number to String and shortens it to 2 decimal
/// places, and adds commas for every thousands
fn format_decimal(number: f64) -> String {
    let mut string = number.to_string();

    let position_of_decimal = string.find('.');

    let mut i: i64 = match position_of_decimal {
        Some(pos) => {
            string.truncate(pos + 3);
            pos as i64
        }
        None => string.len() as i64,
    } - 3;

    while i > 0 {
        string.insert(i as usize, ',');
        i -= 3;
    }

    string
}
