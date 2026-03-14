use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut cnt = 0;

    while let Some(line) = lines.next() {
        let n: usize = match line.unwrap().trim().parse() {
            Ok(num) if num > 0 => num,
            _ => break, // Terminate if n is zero or invalid
        };

        if cnt > 0 {
            println!(); // Print a newline before the next output set
        }

        let mut entries: Vec<(String, i32)> = Vec::new();

        for _ in 0..n {
            let entry_line = lines.next().unwrap().unwrap();
            let parts: Vec<&str> = entry_line.split_whitespace().collect();
            let name = parts[0].to_string();
            let a: i32 = parts[1].parse().unwrap();
            let b: i32 = parts[2].parse().unwrap();
            let c: i32 = parts[3].parse().unwrap();

            // Calculate the score
            let score = a * 3 + b * 0 + c * 1;
            entries.push((name, score));
        }

        // Sort entries by score in descending order
        entries.sort_by(|a, b| b.1.cmp(&a.1));

        // Output the sorted names and scores
        for (name, score) in entries {
            println!("{},{}", name, score);
        }

        cnt += 1;
    }
}
