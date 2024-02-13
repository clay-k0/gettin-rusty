// Topic: Working with an enum
// * print the name of a color to the terminal
//

// * Use an enum with color names as variants
enum Color {
    Green,
    Blue,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(color: Color) {
    // * Use a match expression to determine which color
    //   name to print
    match color {
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

fn main() {
    let mut my_color = Color::Green;

    print_color(my_color);

    my_color = Color::Blue;

    print_color(my_color);
}
