mod utilities;
mod command_mode;

const USER: utilities::User = utilities::User::new(String::new());
fn main() {
    let m = command_mode::CommandMode::new(vec![]);
    m.enter_command();
}
