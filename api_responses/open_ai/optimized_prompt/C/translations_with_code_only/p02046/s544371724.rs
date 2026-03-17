use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok());

        if let (Some(x), Some(y)) = (nums.next(), nums.next()) {
            let x = x.abs();
            let y = y.abs();
            let t;

            if x + y == 1 {
                t = 1;
            } else if (x % 2 != 0) && (y % 2 != 0) {
                t = x + y - 1;
            } else if (x % 2 == 0) && (y % 2 == 0) {
                t = x + y;
            } else if x != 0 && y != 0 {
                t = x + y;
            } else {
                t = x + y + 1;
            }

            println!("{}", t);
        }
    }
}
