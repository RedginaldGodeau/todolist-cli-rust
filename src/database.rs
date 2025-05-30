use rusqlite::{Connection};

pub fn connexion(db_path: String) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(db_path)?;
    Ok(conn)
}