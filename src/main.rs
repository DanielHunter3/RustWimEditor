mod utilities;
mod command_mode;

use utilities::User;
use command_mode::CommandMode;

fn main() {
    let u = Box::new(User::new("sd".to_string()));
    let binding = vec!["ds", "s"];
    let m = CommandMode::new(&binding, u);
    m.enter_command();
}
