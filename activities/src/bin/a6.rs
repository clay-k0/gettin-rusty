// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Do not use break to exit the loop

fn main() {
    // * Use a mutable integer variable
    let mut i: i32 = 5;

    // * Use a while statement
    while i >= 1 {
        // * Print the variable within the while loop
        println!("{:?}", i);
        i -= 1;
    }
    println!("done!");
}
