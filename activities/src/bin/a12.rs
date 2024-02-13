// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
//

// * Use an enum for the box color
enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Brown"),
            Color::Red => println!("Red"),
        }
    }
}

struct Dimensions {
    length: f64,
    width: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("length: {:?}", self.length);
        println!("width: {:?}", self.width);
        println!("depth: {:?}", self.depth);
    }
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    // * Must include dimensions, weight, and color
    weight: f64,
    color: Color,
    dimensions: Dimensions,
}

impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print(&self) {
        println!("Weight: {:?}", self.weight);
        self.color.print();
        self.dimensions.print();
        println!();
    }
}

fn main() {
    let small_dimensions = Dimensions {
        length: 2.0,
        width: 3.0,
        depth: 3.0,
    };

    let small_box = ShippingBox::new(5.0, Color::Brown, small_dimensions);
    small_box.print();

    let large_dimensions = Dimensions {
        length: 32.0,
        width: 23.0,
        depth: 20.0,
    };

    let large_box = ShippingBox::new(25.0, Color::Red, large_dimensions);
    large_box.print();
}
