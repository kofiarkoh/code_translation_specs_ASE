use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let values: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    if values.len() != 3 {
        panic!("Expected exactly three long integer values");
    }

    if values[0] % 2 == 0 || values[1] % 2 == 0 || values[2] % 2 == 0 {
        println!("0");
    } else {
        let mut max_index = 0;
        for i in 1..3 {
            if values[i] > values[max_index] {
                max_index = i;
            }
        }

        // Create a mutable copy of the values to modify
        let mut product_values = values.clone();
        product_values[max_index] = 1;

        let product = product_values.iter().product::<i64>();
        println!("{}", product);
    }
}
