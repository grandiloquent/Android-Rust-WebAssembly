use rusqlite::Connection;
use std::sync::MutexGuard;
use rusqlite::params;

pub fn execute_query_cookie(db: &MutexGuard<Connection>,t:u32) -> String {
    db.query_row("select value from cookie where type = ?", params![t], |row| {
        Ok(row.get(0).unwrap())
    })
    .unwrap_or(String::new())
}
