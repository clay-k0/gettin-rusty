// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//

// * Use a struct for a persons age, name, and favorite color
struct Person {
    // * The color and name should be stored as a String
    name: String,
    age: i32,
    favorite_color: String,
}

// * The name and colors should be printed using a function
fn print(data: &str) {
    println!("{:?}", data)
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            name: String::from("Dan"),
            age: 78,
            favorite_color: String::from("Red"),
        },
        Person {
            name: String::from("Sereia"),
            age: 5,
            favorite_color: String::from("Pink"),
        },
        Person {
            name: String::from("Clayton"),
            age: 20,
            favorite_color: String::from("Green"),
        },
        Person {
            name: String::from("Ashley"),
            age: 40,
            favorite_color: String::from("Blue"),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print(&person.name);
            print(&person.favorite_color);
        }
    }
}
