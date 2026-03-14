use std::io::{self, BufRead};

const N: usize = 10000;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut ans = [0; N];
    let mut i = 0;

    for line in handle.lines() {
        let n = line.expect("Failed to read line");
        if n.is_empty() {
            break; // Break on empty line, similar to EOF behavior
        }

        let mut seq = [0; 10];
        for j in 0..10 {
            seq[j] = n.chars().nth(j).unwrap() as i32 - '0' as i32;
        }

        for j in 0..9 {
            for k in 0..9 - j {
                seq[k] = (seq[k] + seq[k + 1]) % 10;
            }
        }
        ans[i] = seq[0];
        i += 1;

        if i >= N {
            break; // Prevent overflow of the ans array
        }
    }

    for j in 0..i {
        println!("{}", ans[j]);
    }
}
