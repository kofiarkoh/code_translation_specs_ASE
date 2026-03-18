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
    let ans: i64 = if x >= 0 {
        if y >= x {
            -x as i64 + y as i64
        } else if y > 0 {
            x as i64 - y as i64 + 2
        } else if y >= -x {
            x as i64 + y as i64 + 1
        } else {
            -y as i64 - x as i64 + 1
        }
    } else {
        if y >= -x {
            x as i64 + y as i64 + 1
        } else if y > 0 {
            -x as i64 - y as i64 + 1
        } else if y >= x {
            -x as i64 + y as i64
        } else {
            x as i64 - y as i64 + 2
        }
    };

    // Output
    println!("{}", ans);
}
