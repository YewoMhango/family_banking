use iced::{Column, Container, Row, Text};

use super::UserDetails;

use crate::{
    format_decimal, style,
    style::{ACCENT_COLOR, OPEN_SANS, OPEN_SANS_BOLD},
};

use crate::family_banking::Message;

#[derive(Debug, Clone, Default)]
pub struct HomeTabData {
    pub total_shares: f64,
    pub total_debt: f64,
    pub total_loans: f64,
    pub total_cash: f64,
    pub profit: f64,
    pub user_details: Vec<UserDetails>,
}

pub fn render_home_tab(
    total_cash: f64,
    total_debt: f64,
    total_shares: f64,
    profit: f64,
    user_details: Vec<UserDetails>,
) -> Column<'static, Message> {
    Column::new()
        .padding(20)
        .push(render_home_tab_summary(
            total_cash,
            total_debt,
            total_shares,
            profit,
        ))
        .push(if user_details.len() > 0 {
            Row::new()
                .padding(10)
                .push(render_table_column(
                    "Member".to_string(),
                    user_details.iter().map(|user| user.name.clone()).collect(),
                    iced::HorizontalAlignment::Left,
                ))
                .push(render_table_column(
                    "Shares (K)".to_string(),
                    user_details
                        .iter()
                        .map(|user| format!("{}", format_decimal(user.contribution)))
                        .collect(),
                    iced::HorizontalAlignment::Right,
                ))
                .push(render_table_column(
                    "Percentage".to_string(),
                    user_details
                        .iter()
                        .map(|user| format!("{}%", format_decimal(user.percent)))
                        .collect(),
                    iced::HorizontalAlignment::Right,
                ))
                .push(render_table_column(
                    "Debt (K)".to_string(),
                    user_details
                        .iter()
                        .map(|user| format!("{}", format_decimal(user.loan + user.interest)))
                        .collect(),
                    iced::HorizontalAlignment::Right,
                ))
        } else {
            Row::new()
                .push(
                    Text::new("No data yet. You can start entering data through the \"Users\" tab")
                        .font(OPEN_SANS)
                        .size(28)
                        .color(style::GREY),
                )
                .padding(50)
        })
}

pub fn render_home_tab_summary(
    total_cash: f64,
    total_debt: f64,
    total_shares: f64,
    profit: f64,
) -> Column<'static, Message> {
    #[inline]
    fn cash_display(cash: f64) -> Text {
        Text::new(format!("K{}   ", format_decimal(cash)))
            .color(ACCENT_COLOR)
            .size(32)
            .font(OPEN_SANS_BOLD)
    }

    #[inline]
    fn label_display(label: &str) -> Text {
        Text::new(label).size(32).font(OPEN_SANS)
    }

    Column::new()
        .padding(10)
        .push(
            Row::new()
                .push(label_display("Available Cash: "))
                .push(cash_display(total_cash))
                .push(label_display(" Total Debt: "))
                .push(cash_display(total_debt)),
        )
        .push(
            Row::new()
                .push(label_display("Total Shares: "))
                .push(cash_display(total_shares))
                .push(label_display(" Profit: "))
                .push(cash_display(profit)),
        )
}

pub fn render_table_column<T>(
    label: String,
    column_values: Vec<T>,
    h_alignment: iced::HorizontalAlignment,
) -> Column<'static, Message>
where
    T: Into<String> + Clone,
{
    let mut col = Column::new().width(iced::Length::Units(200)).push(
        Container::new(
            Text::new(label)
                .size(28)
                .font(OPEN_SANS_BOLD)
                .width(iced::Length::Fill)
                .horizontal_alignment(h_alignment),
        )
        .width(iced::Length::Fill)
        .padding(10)
        .style(style::TableRow::Header),
    );

    for (i, value) in column_values.iter().enumerate() {
        col = col.push(
            Container::new(
                Text::new(value.clone())
                    .width(iced::Length::Fill)
                    .size(28)
                    .font(OPEN_SANS)
                    .horizontal_alignment(h_alignment),
            )
            .width(iced::Length::Fill)
            .padding(10)
            .style(if i % 2 == 0 {
                style::TableRow::Lighter
            } else {
                style::TableRow::Darker
            }),
        );
    }

    col
}
