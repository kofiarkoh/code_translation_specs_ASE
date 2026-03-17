use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let h: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let w: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let k: i32 = iter.next().unwrap().parse().expect("Invalid input");

    // Ensure inputs are non-negative integers as per the specification
    if h < 0 || w < 0 || k < 0 {
        println!("No");
        return;
    }

    for i in 0..=h {
        for j in 0..=w {
            let now = (w * i) + (h * j) - (2 * i * j);
            if now == k {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
