// Topic: Option
//
// Requirements:
// * Lockers use numbers and are optional for students
//

// * Use a struct containing the student's name and locker assignment
struct Student {
    name: String,
    // * The locker assignment should use an Option<i32>
    locker: Option<i32>,
}

fn main() {
    let clayton = Student {
        name: String::from("Clayton"),
        locker: Some(7),
    };

    println!("Student: {:?}", clayton.name);

    // * Print out the details of a student's locker assignment
    match clayton.locker {
        Some(number) => println!("Locker Assignment: {:?}", number),
        None => println!("Locker Assignment Not Assigned"),
    }
}
