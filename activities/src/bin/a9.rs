// Topic: Data management using tuples
//

// * Use a function that returns a tuple.
fn coordinate() -> (i32, i32) {
    (2, 5)
}

fn main() {
    // * Destructure the return value into two variables.
    let (x, y) = coordinate();

    println!("coord: ({:?}, {:?})", x, y);

    // Print whether the y-value of a cartesian coordinate is
    // greater than 5, less than 5, or equal to 5.
    // Use an if..else if..else block to determine what to print.
    if y > 5 {
        println!("y > 5");
    } else if y < 5 {
        println!("y < 5");
    } else {
        println!("y = 5");
    }
}
