use std::io::stdin;
use crate::task::Status;

pub enum Action {
    Nothing,
    Quit,
    Add,
    Delete,
    ChangeStatus,
}

pub struct UpdateStatusForm {
    pub id: i32,
    pub status: Status,
}

pub struct TaskForm {
    pub name: String,
    pub description: String,
    pub priority: u32,
}

pub fn adding_task_form() -> TaskForm {
    let mut todo_name_input= String::new();
    println!("Name of task:");
    stdin().read_line(&mut todo_name_input).expect("Failed to read line");

    let mut todo_description_input= String::new();
    println!("Description of task:");
    stdin().read_line(&mut todo_description_input).expect("Failed to read line");

    let mut todo_priority_input= String::new();
    println!("Priority of task:");
    stdin().read_line(&mut todo_priority_input).expect("Failed to read line");
    let todo_priority_int: u32 = todo_priority_input.trim().parse().expect("Input not an integer");

    return TaskForm{
        name: todo_name_input.trim().to_string(),
        description: todo_description_input.trim().to_string(),
        priority: todo_priority_int,
    };
}

pub fn select_action_form() -> Action {
    let mut action_input= String::new();
    println!("(0) Quitter\t(1) Ajouter\t(2) Supprimer\t(3) Changer le status");
    stdin().read_line(&mut action_input).expect("Failed to read line");

    let action_int: u32 = action_input.trim().parse().expect("Input not an integer");
    let mut action : Action = Action::Nothing;
    match action_int {
        0 => action = Action::Quit,
        1 => action = Action::Add,
        2 => action = Action::Delete,
        3 => action = Action::ChangeStatus,
        _ => {}
    }

    return action
}

pub fn delete_task_form() -> i32 {
    let mut input= String::new();
    println!("Donnez l'id de la tache a supprimer: ");
    stdin().read_line(&mut input).expect("Failed to read line");

    let input_int: i32 = input.trim().parse().expect("Input not an integer");

    return input_int
}

pub fn change_status_form() -> UpdateStatusForm {
    let mut input_id= String::new();
    println!("Donnez l'id de la tache a changer le status: ");
    stdin().read_line(&mut input_id).expect("Failed to read line");
    let input_int: i32 = input_id.trim().parse().expect("Input not an integer");

    let mut input_status= String::new();
    println!("(0) Pending\t(1) Cancelled\t(2) Completed\t(3) InProgress\t(4) Pending");
    stdin().read_line(&mut input_status).expect("Failed to read line");
    let input_status_int: i32 = input_status.trim().parse().expect("Input not an integer");

    let status = match input_status_int {
        0 => Status::Pending,
        1 => Status::Cancelled,
        2 => Status::Completed,
        3 => Status::InProgress,
        _ => Status::Pending,
    };

    return UpdateStatusForm{
        id: input_int,
        status,
    }
}