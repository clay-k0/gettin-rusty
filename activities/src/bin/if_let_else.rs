enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    let maybe_user = Some("Clayton");

    // using match is still the ultimate approach since it covers all variants
    match maybe_user {
        Some(user) => println!("Username: {:?}", user),
        None => println!("User not found."),
    }

    // however, if let is useful if you are only checking for one certain thing
    if let Some(user) = maybe_user {
        println!("Username: {:?}", user);
    } else {
        println!("User not found.");
    }

    let green = Color::Green;

    if let Color::Green = green {
        println!("It's green.");
    } else {
        println!("It's not green.");
    }
}
