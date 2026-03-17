use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    
    let t1 = iter.next().unwrap();
    let t2 = iter.next().unwrap();
    let a1 = iter.next().unwrap();
    let a2 = iter.next().unwrap();
    let b1 = iter.next().unwrap();
    let b2 = iter.next().unwrap();

    // Check for the "infinity" condition
    if a1 * t1 + a2 * t2 == b1 * t1 + b2 * t2 {
        println!("infinity");
        return;
    }

    // Swap A and B if A's first element is greater than B's first element
    let (mut a1, mut a2, mut b1, mut b2) = if a1 > b1 {
        (a1 + b1, a2 + b2, a1, a2)
    } else {
        (a1, a2, b1, b2)
    };

    let delta_b_a = b1 - a1;
    let delta_a_b = a2 - b2;

    // Calculate the result based on the conditions
    if delta_b_a * t1 > delta_a_b * t2 {
        println!("0");
    } else {
        let denominator = delta_a_b * t2 - delta_b_a * t1;
        let result = delta_b_a * t1 / denominator;

        if delta_b_a * t1 % denominator == 0 {
            println!("{}", result * 2);
        } else {
            println!("{}", result * 2 + 1);
        }
    }
}
