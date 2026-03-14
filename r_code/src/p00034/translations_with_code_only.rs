use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        let mut r = [0; 10];
        let mut f = 0;

        // Read 10 integers from input
        for i in 0..10 {
            let mut input = String::new();
            if reader.read_line(&mut input).is_err() || input.trim().is_empty() {
                f = 1;
                break;
            }
            // Parse the integer, expecting a comma at the end
            if let Some(pos) = input.find(',') {
                let num_str = &input[..pos];
                if let Ok(num) = num_str.trim().parse::<i32>() {
                    r[i] = num;
                } else {
                    f = 1;
                    break;
                }
            } else {
                f = 1;
                break;
            }
        }

        if f == 1 {
            break;
        }

        // Read v1 and v2
        let mut input = String::new();
        if reader.read_line(&mut input).is_err() {
            break;
        }
        let parts: Vec<&str> = input.split(',').collect();
        let v1: i32 = parts[0].trim().parse().unwrap();
        let v2: i32 = parts[1].trim().parse().unwrap();

        // Calculate the sum of r
        let rs: i32 = r.iter().sum();

        // Calculate x
        let x = (v1 as f64) / (v1 + v2) as f64 * (rs as f64);

        let mut cumulative_sum = 0;
        let mut num = 0;

        // Find the first index where cumulative sum exceeds x
        for i in 0..10 {
            cumulative_sum += r[i];
            if x <= cumulative_sum as f64 {
                num = i + 1;
                break;
            }
        }

        // Print the result
        println!("{}", num);
    }
}
