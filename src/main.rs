use std::io::{self, Write};

fn main() {
    print_menu();
}

fn print_menu() -> i32 {
    clear_screen();
    println!("Welcome to your Rust Task Manager!");
    println!("----------------------------------");
    println!("Choose one action:");
    println!("1. Add Task");
    println!("2. Remove Task");
    println!("3. See Tasks");
    println!("4. Edit Task\n");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Not a valid input");

    let option: i32 = option.trim().parse().expect("Enter a number");

    if option < 1 || option > 4 {
        clear_screen();
        println!("Choose a valid option");
    }

    option
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}

// add tasks
// see tasks
// remove tasks
// complete tasks
//
// Tasl:
//  - Name
//  - Description
//  - Status
//  - Start date in (time:date)
//  - End date (time:date)
