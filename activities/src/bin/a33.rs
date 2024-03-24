// Topic: Lifetimes & Functions
//
// Summary:
// Create a program that compares which string is longer (highest length).
//
// Requirements:
// * The comparison must be done using a function named `longest`
// * No data may be copied (cannot use .to_owned() or .to_string())
// * If both strings are the same length, the first one should be returned

fn longest<'a>(word_one: &'a str, word_two: &'a str) -> &'a str {
    if word_two.len() > word_one.len() {
        word_two
    } else {
        word_one
    }
}

fn main() {
    let short = "hello";
    let long = "this is longer";

    println!("{}", longest(short, long));
}
