// Translated from the given C program.
// Preserves behavior of the original algorithm (sum of first `d` primes per input line, stop on 0).
// Notes on translation choices to preserve C semantics:
// - The original used `int`/`long int` and relied on simple arithmetic. We use i32 throughout which
//   corresponds to common 32-bit C `int`/`long` behavior for this program and keeps wraparound semantics
//   via explicit wrapping operations where increments/additions occur.
// - The original stored primes in a fixed-size array of length 1000 and only wrote new primes while
//   index < 1000. We emulate that by pushing primes into a Vec but only while its length < 1000.
// - The original used sqrt((double)i) and cast to long; we emulate that by using f64 sqrt and casting
//   to i32 (floor for positive values), matching the original behavior.
// - Input parsing uses whitespace-separated integers, equivalent to repeated scanf("%d", &d).
//
// This program reads integers from stdin and for each non-zero integer d prints the sum of the first d
// primes on its own line. A 0 terminates processing.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and parse whitespace-separated integers (like repeated scanf("%d", &d)).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut nums = input.split_whitespace().filter_map(|tok| tok.parse::<i32>().ok());

    while let Some(d) = nums.next() {
        // In the original C code, scanf("%d",&d); if(!d) break;
        if d == 0 {
            break;
        }

        // primes storage. Emulate s[1000] with dynamic storage but only store up to 1000 entries.
        let mut primes: Vec<i32> = Vec::with_capacity(1000);
        primes.push(2);
        primes.push(3);

        // n in C was the index of the last found prime, starting at 1 (since s[0]=2, s[1]=3).
        let mut n: i32 = 1;

        // sum uses wrapping arithmetic to emulate C wraparound on overflow.
        let mut sum: i32 = 0;

        if d <= 2 {
            // sum first d primes (if d <= 0 this sums zero elements -> 0)
            for i in 0..d {
                // i ranges over 0..d-1; convert to usize
                let idx = i as usize;
                // In C, accessing s[i] here is defined because s[0] and s[1] were set and d<=2.
                sum = sum.wrapping_add(primes[idx]);
            }
        } else {
            // For d > 2, start with sum = 5 (2 + 3).
            sum = 5;

            // i iterates over odd numbers starting at 5.
            let mut i_val: i32 = 5;
            loop {
                // k = (long int)sqrt((double)i)
                let k = (i_val as f64).sqrt() as i32;

                // Check divisibility by stored primes up to k.
                let mut composite = false;
                for &p in primes.iter() {
                    if p > k {
                        break;
                    }
                    if i_val % p == 0 {
                        composite = true;
                        break;
                    }
                }

                if !composite {
                    // found a prime
                    n = n.wrapping_add(1);
                    if (primes.len() as i32) < 1000 {
                        primes.push(i_val);
                    }
                    sum = sum.wrapping_add(i_val);
                    // In original: if(n==d-1) break;
                    if n == d - 1 {
                        break;
                    }
                }

                // i += 2 (wrapping to emulate C behavior on overflow)
                i_val = i_val.wrapping_add(2);
            }
        }

        // Print sum with newline, matching printf("%d\n", sum) behavior.
        println!("{}", sum);
    }
}
