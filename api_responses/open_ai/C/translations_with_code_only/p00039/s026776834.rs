use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut str_buf = String::new();
    let mut a = [0; 101];

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        str_buf.clear();
        str_buf.push_str(&line);

        let mut s = 0;
        let mut l = 0;

        // Convert characters to their respective values
        for (i, ch) in str_buf.chars().enumerate() {
            match ch {
                'I' => a[i] = 1,
                'V' => a[i] = 5,
                'X' => a[i] = 10,
                'L' => a[i] = 50,
                'C' => a[i] = 100,
                'D' => a[i] = 500,
                _ => a[i] = 1000,
            }
            l = i + 1; // Update length to the last valid index
        }

        // Calculate the total value
        let mut i = 0;
        while i < l {
            if i + 1 < l && a[i] < a[i + 1] {
                s += a[i + 1] - a[i];
                i += 2; // Skip the next character
            } else {
                s += a[i];
                i += 1; // Move to the next character
            }
        }

        println!("{}", s);
    }
}
