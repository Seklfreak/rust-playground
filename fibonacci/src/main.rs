use std::io;

fn main() {
    let mut nth = String::new();

    println!("Please type in which number to calculate:");
    io::stdin()
        .read_line(&mut nth)
        .expect("Failure reading line");

    let nth: u32 = nth.trim().parse().expect("Please type in a valid number");

    println!("Calculating {}-th fibonacci number.", nth);

    let result = fibonacci(nth);

    println!("Result is {}!", result);
}

fn fibonacci(nth: u32) -> u32 {
    if nth <= 1 {
        return nth;
    }

    fibonacci(nth - 1) + fibonacci(nth - 2)
}
