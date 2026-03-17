use std::io;

fn main() {
    // Read inputs from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse input"))
        .collect();

    // Unpack inputs
    let h = inputs[0];
    let w = inputs[1];
    let k = inputs[2];

    // Ensure inputs are non-negative integers as per the specification
    if h < 0 || w < 0 || k < 0 {
        println!("No");
        return;
    }

    // Check for non-negative integers i and j
    let mut found = false;
    for i in 0..=w {
        for j in 0..=h {
            // Calculate the expression
            let expression = (w * i) + (h * j) - (2 * i * j);
            if expression == k {
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    // Output the result based on the evaluation
    if found {
        println!("Yes");
    } else {
        println!("No");
    }
}
