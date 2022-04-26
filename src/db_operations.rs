use rusqlite::{params, Connection, Result};

use crate::main_view::{
    self, debts_tab::DebtsTabData, home_tab::HomeTabData, users_tab::UsersTabData, UserDetails,
};

pub fn get_password(conn: &Connection) -> Option<String> {
    let password_stmt = conn.prepare("SELECT passwordHash FROM admin");

    match password_stmt {
        Ok(mut stmt) => {
            let mut rows = stmt.query([]).unwrap();

            let password_hash = if let Some(row) = rows.next().unwrap() {
                let hash: String = row.get(0).unwrap();
                // println!("Password hash: {}", hash);
                // println!("Password hash length: {}", hash.len());

                if hash == " " {
                    Option::None
                } else {
                    Option::Some(hash)
                }
            } else {
                // println!("No password hash found");
                create_default_admin_row(conn).unwrap();
                Option::None
            };

            password_hash
        }
        _ => {
            // println!("Attempting to initialize database tables...");
            initialize_db_tables(conn).unwrap();
            Option::None
        }
    }
}

pub fn initialize_db_tables(conn: &Connection) -> Result<usize> {
    Ok(conn.execute(
        "
        CREATE TABLE admin (
            id              INTEGER PRIMARY KEY,
            passwordHash    TEXT NOT NULL
        );",
        [],
    )? + conn.execute(
        "
        CREATE TABLE member (
            memberId    INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            share       DECIMAL,
            loan        DECIMAL,
            interest    DECIMAL
        );",
        [],
    )? + conn.execute(
        "
        INSERT INTO member
        VALUES (0, 'Profits', 0, 0, 0);",
        [],
    )? + create_default_admin_row(conn)?)
}

pub fn create_default_admin_row(conn: &Connection) -> Result<usize> {
    conn.execute(
        "
        INSERT INTO admin
        VALUES (1, ' ');",
        [],
    )
}

pub fn store_password(conn: &Connection, password: String) -> Result<usize> {
    conn.execute(
        "UPDATE admin
                        SET passwordHash = ?1
                        WHERE id = 1",
        params![argon2::hash_encoded(
            password.as_bytes(),
            crate::HASH_SALT,
            &argon2::Config::default()
        )
        .unwrap()],
    )
}

#[derive(Clone, Debug)]
struct Member {
    id: i32,
    name: String,
    share: f64,
    loan: f64,
    interest: f64,
}

/// Returns a list of all members with their details from
/// the database, as well as a value for the profit
fn fetch_members(conn: &Connection) -> Result<(Vec<Member>, f64)> {
    let mut stmt = conn.prepare("SELECT memberId, name, share, loan, interest FROM member")?;

    let members: Vec<Member> = stmt
        .query_map([], |row| {
            Ok(Member {
                id: row.get(0)?,
                name: row.get(1)?,
                share: row.get(2)?,
                loan: row.get(3)?,
                interest: row.get(4)?,
            })
        })?
        .map(|person| person.unwrap())
        .collect();

    let profit_member = members
        .iter()
        .find(|member| member.id == 0)
        .unwrap()
        .clone();

    let members: Vec<Member> = members
        .iter()
        .filter(|m| m.id != 0)
        .map(|m| m.clone())
        .collect();

    Ok((members, profit_member.share))
}

pub fn home_tab_data(conn: &Connection) -> Result<HomeTabData> {
    let (members, profit) = fetch_members(conn)?;

    let (total_shares, total_debt, total_loans) =
        members.iter().fold((0.0, 0.0, 0.0), |acc, member| {
            (
                acc.0 + member.share,
                acc.1 + member.loan + member.interest,
                acc.2 + member.loan,
            )
        });

    Ok(HomeTabData {
        total_shares,
        total_debt,
        total_loans,
        total_cash: total_shares - total_loans + profit,
        profit: profit,
        user_details: members
            .iter()
            .map(|member| UserDetails {
                id: member.id,
                name: member.name.clone(),
                contribution: member.share,
                percent: (member.share / total_shares) * 100.0,
                loan: member.loan,
                interest: member.interest,
            })
            .collect(),
    })
}

pub fn users_tab_data(conn: &Connection) -> Result<UsersTabData> {
    let (members, _) = fetch_members(conn)?;

    let total_shares = members.iter().fold(0.0, |acc, member| acc + member.share);

    Ok(UsersTabData::new(
        members
            .iter()
            .map(|member| UserDetails {
                id: member.id,
                name: member.name.clone(),
                contribution: member.share,
                percent: (member.share / total_shares) * 100.0,
                loan: member.loan,
                interest: member.interest,
            })
            .collect(),
    ))
}

pub fn debts_tab_data(conn: &Connection) -> Result<DebtsTabData> {
    let (members, _) = fetch_members(conn)?;

    let total_shares = members.iter().fold(0.0, |acc, member| acc + member.share);

    Ok(DebtsTabData::new(
        members
            .iter()
            .map(|member| main_view::UserDetails {
                id: member.id,
                name: member.name.clone(),
                contribution: member.share,
                percent: (member.share / total_shares) * 100.0,
                loan: member.loan,
                interest: member.interest,
            })
            .collect(),
    ))
}

pub fn store_new_user(conn: &Connection, name: String, shares: f64) -> Result<usize> {
    conn.execute(
        "INSERT INTO member (name, share, loan, interest) VALUES (?1, ?2, 0, 0);",
        params![name, shares],
    )
}

pub fn edit_user(conn: &Connection, id: i32, name: String, shares: f64) -> Result<usize> {
    conn.execute(
        "
        UPDATE member
        SET name = ?2, share = ?3
        WHERE memberId = ?1;",
        params![id, name, shares],
    )
}

pub fn delete_user(conn: &Connection, id: i32) -> Result<usize> {
    conn.execute(
        "
        DELETE FROM member
        WHERE memberId = ?1;",
        params![id],
    )
}

pub fn borrow_debt(conn: &Connection, user_id: i32, loan: f64, interest: f64) -> Result<usize> {
    conn.execute(
        "
        UPDATE member
        SET loan = loan + ?2, interest = interest + ?3
        WHERE memberId = ?1;",
        params![user_id, loan, interest],
    )
}

pub fn repay_debt(conn: &Connection, user_id: i32, loan: f64, interest: f64) -> Result<usize> {
    Ok(conn.execute(
        "
        UPDATE member
        SET loan = loan - ?2, interest = interest - ?3
        WHERE memberId = ?1;",
        params![user_id, loan, interest],
    )? + conn.execute(
        "
        UPDATE member
        SET share = share + ?1
        WHERE memberId = 0;",
        params![interest],
    )?)
}
