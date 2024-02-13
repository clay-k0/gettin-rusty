// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//

fn compare_to_five(number: i32) {
    // * Use an if..else if..else block to determine which message to display
    // * Use the println macro to display messages to the terminal
    if number > 5 {
        println!("{} > 5", number)
    } else if number < 5 {
        println!("{} < 5", number)
    } else {
        println!("{} = 5", number)
    }
}

fn main() {
    // * Use a variable set to any integer value
    let value: i32 = 5;

    compare_to_five(value);
}
