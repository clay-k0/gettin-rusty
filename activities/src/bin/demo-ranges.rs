fn main() {
    let inclusive_range = 1..=3; // inclusive, contains 1-3
    let exclusive_range = 1..4; // exclusive, contains 1-3

    for num in inclusive_range {
        println!("{:?}", num);
    }
    for num in exclusive_range {
        println!("{:?}", num);
    }

    for num in 1..=10 {
        println!("{:?}", num);
    }

    for ch in 'a'..='d' {
        println!("{:?}", ch);
    }

    let under_five: Vec<u8> = (1..=5).filter(|&num| num < 5).collect();

    println!("{:?}", under_five);
}
