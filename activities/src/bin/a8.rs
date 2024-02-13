// Topic: Organizing similar data using structs
// * Print the flavor of a drink and it's fluid ounces
//

// * Use an enum to create different flavors of drinks
enum Flavor {
    Cherry,
    Strawberry,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    ounces: f64,
}

// * Use a function to print out the drink flavor and ounces
fn print_drink_info(drink: Drink) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Cherry => println!("The drink flavor is strawberry."),
        Flavor::Strawberry => println!("The drink flavor is cherry."),
    }

    println!("The drink is {:?} oz.", drink.ounces);
}

fn main() {
    let cherry_drink = Drink {
        flavor: Flavor::Cherry,
        ounces: 12.2,
    };

    print_drink_info(cherry_drink);

    let strawberry_drink = Drink {
        flavor: Flavor::Strawberry,
        ounces: 24.7,
    };

    print_drink_info(strawberry_drink);
}
