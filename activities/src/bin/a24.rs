// Topic: Iterator
//
// * Use an iterator chain to accomplish the task.

fn main() {
    // * Triple the value of each item in a vector.
    // * Filter the data to only include values > 10.
    let data: Vec<_> = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|num| num * 3)
        .filter(|num| num > &10)
        .collect();

    // * Print out each element using a for loop.
    for number in data {
        println!("{:?}", number);
    }
}
