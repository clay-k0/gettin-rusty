// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//

fn main() {
    // * Use a variable set to either true or false
    let greet: bool = true;

    // * Use an if..else block to determine which message to display
    // * Use the println macro to display messages to the terminal
    if greet {
        println!("hello");
    } else {
        println!("goodbye")
    }
}
