fn main() {
    let mut data = Some(3);

    while let Some(_) = data {
        println!("loop");
        data = None;
    }
    println!("done");

    let numbers: Vec<_> = (1..=3).collect();
    let mut numbers_iter = numbers.iter();

    while let Some(num) = numbers_iter.next() {
        println!("num = {:?}", num);
    }
}
