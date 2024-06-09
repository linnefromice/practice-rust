use std::process::Command;

fn main() {
    check_command("ls");
    check_command("dfx --version");
    // This is not a good idea, as the command will be executed.
}

fn check_command(command: &str) {
    match Command::new(command).spawn() {
        Ok(child) => {
            println!("Command `{}` is available", command);
            println!("{:?}", child);
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}