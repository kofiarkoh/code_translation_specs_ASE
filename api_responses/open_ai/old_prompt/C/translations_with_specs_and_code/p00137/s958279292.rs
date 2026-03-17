use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let counter: i32 = input.trim().parse().expect("Invalid input for counter");

    if counter <= 0 {
        return; // No output for non-positive counter
    }

    for count in 0..counter {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut x: i32 = input.trim().parse().expect("Invalid input for x");

        const CTER: i32 = 10;
        println!("Case {}:", count + 1);

        for _ in 0..CTER {
            x = (x * x) / 100 % 10000; // Perform the squaring, division, and modulo
            println!("{}", x);
        }
    }
}
