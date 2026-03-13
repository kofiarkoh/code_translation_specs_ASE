use std::io::{self, Read};

fn main() {
    // Read all input from stdin. Spec assumes well-formed integer pairs and a terminating "0 0".
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse whitespace-separated integers.
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    // Process pairs (n, m) until the terminating pair 0 0.
    loop {
        let n = match tokens.next() {
            Some(x) => x,
            None => break, // EOF before terminating pair: stop (spec leaves this undefined; safe stop).
        };
        let m = match tokens.next() {
            Some(x) => x,
            None => break,
        };

        // Termination condition: do not process the pair 0 0.
        if n == 0 && m == 0 {
            break;
        }

        // Spec preconditions guarantee 1 <= n <= 1000 and m >= 1 for processed pairs.
        let n_usize = n as usize;

        // d[i] == 1 means alive, 0 means removed.
        let mut d = vec![1u8; n_usize];

        // i: current index (0-based), sum: current count of alive positions encountered,
        // f: count of removed positions so far.
        let mut i: usize = 0;
        let mut sum: i32 = 0;
        let mut f: i32 = 0;

        // Simulate elimination exactly as in the specification/C code:
        loop {
            sum += d[i] as i32;
            if sum == m {
                d[i] = 0;
                sum = 0;
                f += 1;
            }
            // If all positions removed, i is the index of the last removed element.
            if f == n {
                break;
            }
            // Advance i with wrap-around.
            if i == n_usize - 1 {
                i = 0;
                continue;
            }
            i += 1;
        }

        // Print 1-based index of the last eliminated position followed by a newline.
        println!("{}", i + 1);
    }
}
