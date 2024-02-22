// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calculate_perimeter(&self) -> f64;
}

struct Square {
    side: f64,
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> f64 {
        self.side * 4.0
    }
}

struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
}

fn perimeter(shape: impl Perimeter) {
    let perimeter = shape.calculate_perimeter();
    println!("The perimeter is {:?}", perimeter);
}

fn main() {
    perimeter(Square { side: 5.0 });
    perimeter(Triangle {
        side_a: 2.0,
        side_b: 12.0,
        side_c: 5.0,
    });
}
