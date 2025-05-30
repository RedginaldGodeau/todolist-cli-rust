use std::os::macos::raw::time_t;
use std::time::{SystemTime, UNIX_EPOCH};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub enum Status {
    Completed,
    InProgress,
    Pending,
    Cancelled,
}

pub fn to_string(status: Status) -> &'static str {
    return match status {
        Status::Completed => "Completed",
        Status::InProgress => "InProgress",
        Status::Pending => "Pending",
        Status::Cancelled => "Cancelled",
    };
}

pub fn to_status(status: &str) -> Status {
    return match status {
        "Completed" => Status::Completed,
        "InProgress" => Status::InProgress,
        "Pending" => Status::Pending,
        "Cancelled" => Status::Cancelled,
        _ => Status::Pending,
    };
}
#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub priority: u32,
    pub status: Status,
    pub create_at: time_t,
    pub date: time_t,
}

pub fn schema(conn: &Connection) -> Result<()> {
    conn.execute("CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    priority INTEGER NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('Completed', 'InProgress', 'Pending', 'Cancelled')),
    create_at INTEGER NOT NULL,
    date INTEGER NOT NULL
);", [])?;
    Ok(())
}

pub fn create_task(conn: &Connection, name: String, description: String, date: time_t, priority: u32, status: Status) -> Result<()> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as time_t;

    let status_str= to_string(status);

    conn.execute(
        "INSERT INTO tasks (name, description, priority, status, create_at, date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![name, description, priority, status_str, current_time, date],
    )?;

    Ok(())
}

pub fn select_all(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, name, description, priority, status, create_at, date FROM tasks")?;

    let task_iter = stmt.query_map([], |row| {
        let status_str: String = row.get(4)?;
        let status = to_status(&*status_str);

        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            priority: row.get(3)?,
            status,
            create_at: row.get(5)?,
            date: row.get(6)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }

    Ok(tasks)
}

pub fn delete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?", params![id])?;
    Ok(())
}

pub fn update_status_task(conn: &Connection, id: i32, status: Status) -> Result<()> {
    let status_str = to_string(status);
    conn.execute(
        "UPDATE tasks SET status = ?1 WHERE id = ?2",
        [&status_str, &&**&id.to_string()],
    )?;
    Ok(())
}