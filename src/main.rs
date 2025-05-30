use std::os::unix::raw::time_t;
use std::time::{SystemTime, UNIX_EPOCH};

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
            Err(_) => return
        };

        print!("{}[2J", 27 as char);
        print!("ID\t| Nom\t| Description\t| Priorité\t| Status\n");
        for task in tasks {
            println!("{}\t| {}\t| {}\t| {}\t\t| {}\n", task.id, task.name, task.description, task.priority, task::to_string(task.status));
        }
        print!("-------------------------------------------\n");
        match form::select_action_form() {
            form::Action::Nothing => {}
            form::Action::Quit => starting = false,
            form::Action::Delete => {
                let task_id = form::delete_task_form();
                task::delete_task(&conn, task_id);
            }
            form::Action::Add => {
                let task_form = form::adding_task_form();
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as time_t;
                let _ = task::create_task(&conn, task_form.name, task_form.description, current_time, task_form.priority, task::Status::Pending);
            }
            form::Action::ChangeStatus => {
                let task_form = form::change_status_form();
                let _ = task::update_status_task(&conn, task_form.id, task_form.status);
            }
            _ => {}
        }

    }
}
