// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
//
// Notes:
// * Use the .len() function to print the number of elements in a vector

fn main() {
    // * Use a vector to store 4 numbers
    let my_numbers = vec![10, 20, 30, 40];

    // * Iterate through the vector using a for..in loop
    for number in &my_numbers {
        // * Determine whether to print the number or print "thirty" inside the loop
        match number {
            30 => println!("thirty"),
            _ => println!("{:?}", number),
        }
    }

    // * Print the total number of elements in a vector
    println!("Number of Elements: {:?}", my_numbers.len());
}
