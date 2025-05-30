use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Local};

mod form;
mod database;
mod task;

fn main() {
    let conn = match database::connexion("data.db".parse().unwrap()) {
        Ok(conn) => conn,
        Err(_) => return
    };
    match task::schema(&conn) {
        Ok(_) => {},
        Err(_) => println!("Error creating schema")
    }
    let mut starting = true;

    while starting {
        let tasks = match task::select_all(&conn) {
            Ok(tasks) => tasks,
            Err(err) => { println!("{}", err); return;}
        };

        println!("{}[2J", 27 as char);
        println!("ID\t| Nom\t| Description\t| Priorité\t| Status\t| Date\n");
        for task in tasks {
            let system_time = UNIX_EPOCH + Duration::from_secs(task.date as u64);
            let datetime: DateTime<Local> = system_time.into();
            println!("{}\t| {}\t| {}\t| {}\t\t| {}\t| {}", task.id, task.name, task.description, task.priority, task::to_string(task.status), datetime.format("%d-%m-%Y").to_string());
        }
        println!("-------------------------------------------");
        match form::select_action_form() {
            form::Action::Nothing => {}
            form::Action::Quit => starting = false,
            form::Action::Delete => {
                let task_id = form::delete_task_form();
                match task::delete_task(&conn, task_id) {
                    Ok(_) => {},
                    Err(err) => {println!("{}", err); return;}
                };
            }
            form::Action::Add => {
                let task_form = form::adding_task_form();
                match task::create_task(&conn, task_form.name, task_form.description, task_form.priority, task::Status::Pending) {
                    Ok(_) => {},
                    Err(err) => {println!("{}", err); return;}
                };
            }
            form::Action::ChangeStatus => {
                let task_form = form::change_status_form();
                match task::update_status_task(&conn, task_form.id, task_form.status) {
                    Ok(_) => {},
                    Err(err) => {println!("{}", err); return;}
                };
            }
        }

    }
}
