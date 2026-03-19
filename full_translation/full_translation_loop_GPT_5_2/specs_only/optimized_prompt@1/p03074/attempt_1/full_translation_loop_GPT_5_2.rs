use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    if n == 0 {
        // Spec: if first integer is zero, return zero.
        println!("0");
        return;
    }

    let s = match it.next() {
        Some(v) => v.as_bytes(),
        None => {
            // Spec does not define behavior if string missing; keep narrow behavior.
            println!("0");
            return;
        }
    };

    // Spec: if size of string is less than specified length, behavior is not defined.
    // We avoid panicking by clamping processing to available bytes, but this is a best-effort.
    let m = n.min(s.len());

    // Sliding window: maximum length of substring with at most k zeros (flip them to ones).
    let mut left: usize = 0;
    let mut zeros: usize = 0;
    let mut best: usize = 0;

    for right in 0..m {
        if s[right] == b'0' {
            zeros += 1;
        }
        while zeros > k && left <= right {
            if s[left] == b'0' {
                zeros -= 1;
            }
            left += 1;
        }
        let len = right + 1 - left;
        if len > best {
            best = len;
        }
    }

    println!("{}", best);
}
