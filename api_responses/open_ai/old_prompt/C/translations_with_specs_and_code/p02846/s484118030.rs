use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace().map(|s| s.parse::<i64>().expect("Invalid input"));
    
    let t1 = iter.next().unwrap();
    let t2 = iter.next().unwrap();
    let a1 = iter.next().unwrap();
    let a2 = iter.next().unwrap();
    let b1 = iter.next().unwrap();
    let b2 = iter.next().unwrap();

    // Check for equality of linear combinations
    if a1 * t1 + a2 * t2 == b1 * t1 + b2 * t2 {
        println!("infinity");
        return;
    }

    // Ensure A[1] is less than or equal to B[1]
    let (mut a1, mut a2, mut b1, mut b2) = if a1 > b1 {
        let temp_a1 = a1 + b1;
        let temp_b1 = a1;
        a1 = temp_a1 - b1;
        b1 = temp_b1;
        
        let temp_a2 = a2 + b2;
        let temp_b2 = a2;
        a2 = temp_a2 - b2;
        b2 = temp_b2;

        (a1, a2, b1, b2)
    } else {
        (a1, a2, b1, b2)
    };

    let numerator = b1 - a1;
    let denominator = (a2 - b2) * t2 - (b1 - a1) * t1;

    if numerator * t1 > denominator {
        println!("0");
    } else if denominator == 0 {
        // This case should not happen due to the previous equality check
        println!("0");
    } else if numerator * t1 % denominator == 0 {
        println!("{}", (numerator * t1 / denominator) * 2);
    } else {
        println!("{}", (numerator * t1 / denominator) * 2 + 1);
    }
}
