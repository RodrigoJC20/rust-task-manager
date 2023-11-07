use std::io::{self, Write};

struct Task {
    name: String,
    description: String,
    status: i32,
}

/*
* Task:
*   - Name
*   - Description
*   - Status (Pending, In progress, Completed)
*/

// Operations:
// add tasks
// see tasks
// remove tasks
// complete tasks

fn main() {
    let mut pending: Vec<Task> = Vec::new();
    let mut in_progress: Vec<Task> = Vec::new();
    let mut completed: Vec<Task> = Vec::new();

    let mut action: i32 = 1;

    while action != 5 {
        action = print_menu();
        clear_screen();
        match action {
            1 => {
                add_task(&mut pending);
                wait_for_enter();
            }
            2 => {
                println!("Removing task...");
                wait_for_enter();
            }
            3 => {
                print_tasks(&pending);
                wait_for_enter();
            }
            4 => {
                println!("Editing task....");
                wait_for_enter();
            }
            _ => {
                println!("Goodbye!");
                wait_for_enter();
            }
        }
    }
}

fn add_task(pending: &mut Vec<Task>) {
    let mut input = String::new();

    println!("Name:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let name: String = input.trim().parse().expect("Failed to parse the input");

    println!("Description:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let description: String = input.trim().parse().expect("Failed to parse the input");

    let new_task = Task {
        name,
        description,
        status: 1,
    };

    pending.push(new_task);
}

fn remove_task(task_list: &mut Vec<Task>) {
    let mut input = String::new();

    print!("Task Name:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    for i in o..task_list.len() {
        // Use a list instead of vector
    }
}

fn print_tasks(pending: &Vec<Task>) {
    if pending.is_empty() {
        println!("No pending tasks.");
    } else {
        for task in pending {
            println!("- {}:", task.name);
            println!("  {}", task.description);
        }
    }
}

fn print_menu() -> i32 {
    let mut action: i32 = 0;

    while action < 1 || action > 5 {
        clear_screen();
        println!("Welcome to your Rust Task Manager!");
        println!("----------------------------------");
        println!("Choose one action:");
        println!("1. Add Task");
        println!("2. Remove Task");
        println!("3. See Tasks");
        println!("4. Edit Task");
        println!("5. Exit\n");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input");

        action = option.trim().parse().expect("Enter a number");

        if action < 1 || action > 5 {
            clear_screen();
            println!("Choose a valid option");
            wait_for_enter();
        }
    }

    action
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}

fn wait_for_enter() {
    println!("\nPress enter to continue...");

    let mut wait = String::new();

    io::stdin()
        .read_line(&mut wait)
        .expect("Failed to read line");
}
