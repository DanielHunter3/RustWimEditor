extern crate color_print;

use std::io::{stdin, Write};

use crate::utilities::User;
use color_print::{cprint, cprintln};

pub struct CommandMode<'a> {
    commands: Box<Vec<&'a str>>,
    user: Box<User>
}

impl<'a> CommandMode<'a> {
    pub fn new(vector: &'a Vec<&str>, user: Box<User>) -> CommandMode<'a> {
        CommandMode { commands: Box::new(vector.to_vec()), user }
    }

    #[allow(dead_code)]
    pub fn see_commands(&self) {
        let mut i: u8 = 0;
        for command in &*self.commands {
            i += 1;
            println!("{}. {}", i, command);
        }
    }

    pub fn enter_command(&self) {
        let is_admin: &str = if self.user.is_admin() {"#"} else {"$"};
        cprint!(
            "<green>{}</>@<magenta>{}</>:<yellow>~{}</> ",
            self.user.get_name(),
            gethostname::gethostname().to_string_lossy(), // Convert Option<&str> to &str
            is_admin
        );
        
        let command = input();

        if !in_vector(&self.commands, &command.as_str()) {
            cprintln!("Invalid command: <red>{}</>", command);
            return;
        }
        match command.as_str() {
            "open" => {},
            _ => cprintln!("<red>Executing command: {}</red>", command),
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
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input
}