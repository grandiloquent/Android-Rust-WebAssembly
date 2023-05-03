use rusqlite::Connection;
use std::sync::MutexGuard;

pub fn execute_query_cookie(db: &MutexGuard<Connection>) -> String {
    db.query_row("select value from cookie where type = 5", [], |row| {
        Ok(row.get(0).unwrap())
    })
    .unwrap_or(String::new())
}
