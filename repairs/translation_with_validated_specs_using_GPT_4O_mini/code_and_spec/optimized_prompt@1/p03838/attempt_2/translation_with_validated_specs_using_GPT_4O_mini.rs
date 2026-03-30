use std::io;

fn main() {
    // Input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut nums = input.split_whitespace();

    // Safely parse x and y, handling potential parsing errors
    let x: i32 = match nums.next().and_then(|s| s.parse().ok()) {
        Some(value) => value,
        None => {
            eprintln!("Invalid input for x");
            return;
        }
    };

    let y: i32 = match nums.next().and_then(|s| s.parse().ok()) {
        Some(value) => value,
        None => {
            eprintln!("Invalid input for y");
            return;
        }
    };

    // Solve
    let ans: i64; // Using i64 to represent long long in C
    if x >= 0 {
        if y >= x {
            ans = (y - x) as i64;
        } else if y > 0 {
            ans = (x - y + 2) as i64;
        } else if y >= -x {
            ans = (x + y + 1) as i64;
        } else {
            ans = (-y - x + 1) as i64;
        }
    } else {
        if y >= -x {
            ans = (x + y + 1) as i64;
        } else if y > 0 {
            ans = (-x - y + 1) as i64;
        } else if y >= x {
            ans = (y - x) as i64;
        } else {
            ans = (x - y + 2) as i64;
        }
    }

    // Output
    println!("{}", ans);
}
