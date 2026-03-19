use std::io::{self, Write};

fn main() {
    // Input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let x: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let y: i32 = iter.next().unwrap().parse().expect("Invalid input");

    // Solve
    let ans: i64; // Use i64 to match the C long long type
    if x >= 0 {
        if y >= x {
            ans = -x as i64 + y as i64;
        } else if y > 0 {
            ans = x as i64 - y as i64 + 2;
        } else if y >= -x {
            ans = x as i64 + y as i64 + 1;
        } else {
            ans = -y as i64 - x as i64 + 1;
        }
    } else {
        if y >= -x {
            ans = x as i64 + y as i64 + 1;
        } else if y > 0 {
            ans = -x as i64 - y as i64 + 1;
        } else if y >= x {
            ans = -x as i64 + y as i64;
        } else {
            ans = x as i64 - y as i64 + 2;
        }
    }

    // Output
    print!("{}", ans);
    io::stdout().flush().expect("Failed to flush stdout");
}
