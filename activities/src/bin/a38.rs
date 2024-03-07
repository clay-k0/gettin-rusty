// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn msg_hello() -> &'static str {
    thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let first_part: JoinHandle<&'static str> = thread::spawn(move || msg_hello());
    let second_part: JoinHandle<&'static str> = thread::spawn(move || msg_thread());
    let third_part: JoinHandle<&'static str> = thread::spawn(move || msg_excited());

    let first_result = first_part.join().expect("failed to join at first_result");
    let second_result = second_part.join().expect("failed to join at second_result");
    let third_result = third_part.join().expect("failed to join at third_result");

    println!("{}{}{}", first_result, second_result, third_result);
}
