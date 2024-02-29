#[allow(dead_code)]
// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct Shoes(Color);
impl Shoes {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct Shirt(Color);
impl Shirt {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct Pants(Color);
impl Pants {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

fn display_shoes_color(shoes: Shoes) {
    println!("{:?}", shoes)
}

fn display_shirt_color(shirt: Shirt) {
    println!("{:?}", shirt)
}

fn display_pants_color(pants: Pants) {
    println!("{:?}", pants)
}

fn main() {
    let blue_shoes = Shoes::new(Color::Blue);
    let purple_shirt = Shirt::new(Color::Purple);
    let pink_pants = Pants::new(Color::Custom(String::from("Pink")));

    display_shoes_color(blue_shoes);
    display_shirt_color(purple_shirt);
    display_pants_color(pink_pants);
}
