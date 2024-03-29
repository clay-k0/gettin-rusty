// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn print_action(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("Turning Off..."),
        Sleep => println!("Sleeping..."),
        Reboot => println!("Rebooting..."),
        Shutdown => println!("Shutting down..."),
        Hibernate => println!("Hibernating..."),
    }
}

fn convert_state(input: &str) {
    match input.to_lowercase().as_str() {
        "off" => print_action(PowerState::Off),
        "sleep" => print_action(PowerState::Sleep),
        "reboot" => print_action(PowerState::Reboot),
        "shutdown" => print_action(PowerState::Shutdown),
        "hibernate" => print_action(PowerState::Hibernate),
        _ => println!("Power state not found."),
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {
    println!("Enter Power State: ");
    let input_state = get_input();

    match input_state {
        Ok(message) => convert_state(&message),
        Err(e) => println!("Error: {:?}", e),
    }
}
