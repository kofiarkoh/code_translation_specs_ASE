use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok());

        if let (Some(a), Some(b)) = (nums.next(), nums.next()) {
            let (mut a, mut b) = if b > a { (b, a) } else { (a, b) };
            let mut c = 1;
            let mut tmp_a = a;
            let mut tmp_b = b;

            for i in 2..=tmp_b {
                while tmp_b % i == 0 && tmp_a % i == 0 {
                    c *= i;
                    tmp_b /= i;
                    tmp_a /= i;
                }
            }

            // Output the result, ensuring the format matches the C code
            println!("{} {}", c, tmp_a * tmp_b * c);
        }
    }
}
