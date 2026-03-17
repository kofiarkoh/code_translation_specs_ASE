use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace().map(|s| s.parse::<i64>().expect("Invalid input"));
    
    let t1 = iter.next().unwrap();
    let t2 = iter.next().unwrap();
    
    let mut a1 = iter.next().unwrap();
    let mut a2 = iter.next().unwrap();
    
    let mut b1 = iter.next().unwrap();
    let mut b2 = iter.next().unwrap();
    
    if a1 * t1 + a2 * t2 == b1 * t1 + b2 * t2 {
        println!("infinity");
        return;
    }
    
    if a1 > b1 {
        // Swap A and B
        std::mem::swap(&mut a1, &mut b1);
        std::mem::swap(&mut a2, &mut b2);
    }
    
    let delta_a = b1 - a1;
    let delta_b = a2 - b2;
    
    if delta_a * t1 > delta_b * t2 {
        println!("0");
    } else {
        let denominator = delta_b * t2 - delta_a * t1;
        let result = if delta_a * t1 % denominator == 0 {
            delta_a * t1 / denominator * 2
        } else {
            delta_a * t1 / denominator * 2 + 1
        };
        println!("{}", result);
    }
}
