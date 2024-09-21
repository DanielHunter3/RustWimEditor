extern crate color_print;

use std::ffi::OsString;
use std::io::stdin;

use crate::utilities::User;
use crate::USER;

pub struct CommandMode<'a> {
    commands: Box<Vec<&'a str>>,
    user: User
}

impl<'a> CommandMode<'a> {
    pub fn new(vector: Vec<&str>) -> CommandMode {
        CommandMode { commands: Box::new(vector), user: USER }
    }

    pub fn see_commands(&self) {
        let mut i: u8 = 0;
        for command in &*self.commands {
            i += 1;
            println!("{}. {}", i, command);
        }
    }

    pub fn enter_command(&self) {
        let is_admin: &str = if USER.is_admin() {"#"} else {"$"};
        color_print::cprint!("<green>{}</>@<magenta>{:?}</>:<yellow>~{}</> ", USER.name, gethostname::gethostname(), is_admin);
        let command = input();

        if !in_vector(&self.commands, &command.as_str()) {
            println!("Invalid command: {command}");
            return;
        }
        match command.as_str() {
            "open" => {},
            _ => color_print::cprintln!("<red>Executing command: {}</red>", command),
        }
    }
}

fn in_vector<T: Eq>(vector: &Vec<T>, element: &T) -> bool {
    for i in vector {
        if i == element {
            return true;
        }
    }
    false
}

pub fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input
}