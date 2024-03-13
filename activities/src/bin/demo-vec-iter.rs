fn main() {
    let numbers: Vec<_> = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
    println!("Vec: {:?}\n", numbers);

    let plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();
    println!("+1: {:?}", plus_one);

    let filtered_numbers: Vec<_> = numbers.iter().filter(|num| num > &&2).collect();
    println!(">2: {:?}", filtered_numbers);

    let find_number = numbers.iter().find(|num| num == &&9);
    println!("==9: {:?}", find_number);

    let count = numbers.iter().count();
    println!("Count: {:?}", count);

    let last = numbers.iter().last();
    println!("Last: {:?}", last);

    let min = numbers.iter().min();
    let max = numbers.iter().max();
    println!("Min: {:?} | Max: {:?}", min, max);

    let take: Vec<&i32> = numbers.iter().take(3).collect();
    println!("Take 3: {:?}", take);

    let chained_filter: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        .filter(|num| num > &1)
        .collect();

    println!("+1 && >1: {:?}", chained_filter);
}
