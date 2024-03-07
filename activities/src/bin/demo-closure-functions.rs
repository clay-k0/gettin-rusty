fn math(a: u8, b: u8, op: Box<dyn Fn(u8, u8) -> u8>) -> u8 {
    op(a, b)
}

fn main() {
    let name = "clay";

    let add = Box::new(move |a: u8, b: u8| {
        println!("adding for {}!", name);
        a + b
    });
    let sub = Box::new(|a: u8, b: u8| a - b);
    let div = Box::new(|a: u8, b: u8| a / b);
    let mul = Box::new(|a: u8, b: u8| a * b);
    let modulus = Box::new(|a: u8, b: u8| a % b);

    println!("{}", math(12, 10, add));
    println!("{}", math(5, 2, sub));
    println!("{}", math(100, 20, div));
    println!("{}", math(10, 10, mul));
    println!("{}", math(30, 3, modulus));
}
