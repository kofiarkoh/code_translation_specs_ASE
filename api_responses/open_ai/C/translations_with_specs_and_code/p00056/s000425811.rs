// translate_goldbach.rs
//
// Translated from the provided C program and guided by the SPEC.
// Behavior:
//  - Precomputes primality info up to 5000 using the same sieve logic as the C code.
//  - Reads whitespace-separated integers from stdin until a 0 is encountered (0 not processed).
//  - For each non-zero integer n, counts unordered ways to write n as a sum of two primes
//    (primes may be equal). Writes one integer per line to stdout.
//
// Notes on choices preserving/specifying behavior:
//  - The SPEC requires correct results for n <= 5000. The original C code only
//    initializes primality up to 5000. To avoid UB/crashes in Rust, this translation
//    limits prime lookups to indices <= 5000. For n > 5000 the result is therefore
//    computed using only known primes up to 5000 (the SPEC states behavior for n>5000
//    is unspecified).
//  - Malformed tokens cause parsing to stop (the SPEC marks malformed input handling as
//    unspecified for the original C; here we stop processing on parse error).
//  - Negative and small n (< 2) produce the output 0 as in the original logic.
//
// Implementation is safe, idiomatic Rust and avoids unsafe.

use std::io::{self, Read};

fn main() {
    // Prepare composite[] array sized for indices 0..=5000.
    // In the original C code `prime[i] == 0` meant "is prime", and `1` meant "composite".
    // Here `composite[i] == true` means composite (non-prime).
    let mut composite = vec![false; 5001usize];

    // Mark even numbers >= 4 as composite (same as C: for(i=4;i<=5000;i+=2) prime[i]=1;)
    let mut i = 4usize;
    while i <= 5000 {
        composite[i] = true;
        i += 2;
    }

    // Sieve odd prime multiples, following the C code:
    // for(i=3;i*i<=5000;i+=2){ if(prime[i]==0) for(o=i*2;o<=5000;o+=i) prime[o]=1; }
    let mut i = 3usize;
    while i * i <= 5000 {
        if !composite[i] {
            let mut o = i * 2;
            while o <= 5000 {
                composite[o] = true;
                o += i;
            }
        }
        i += 2;
    }

    // Read all input from stdin as whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, just exit silently (mirrors unspecified behaviour of C on I/O errors).
        return;
    }

    let mut output = String::new();

    for token in input.split_whitespace() {
        // If a token fails to parse as an integer, stop processing further tokens.
        // The SPEC treats malformed input as unspecified; we choose to stop.
        let num: i32 = match token.parse() {
            Ok(n) => n,
            Err(_) => break,
        };

        // Termination condition: stop processing when a 0 is read (0 is not processed).
        if num == 0 {
            break;
        }

        let mut count: i32 = 0;

        // For n < 2, there are no prime pairs; count remains 0.
        if num >= 2 {
            // To avoid out-of-bounds accesses, only iterate primes up to 5000 (the precomputed limit).
            // This reflects the SPEC's precondition that correct results are guaranteed only for n <= 5000.
            let max_idx = std::cmp::min(5000, num) as usize;

            // Reproduce the C nested loop semantics:
            // for(i=2;i<=num;i++)
            //   if(!prime[i]){
            //     for(o=2;o<=num;o++)
            //       if(!prime[o]){
            //         sum=i+o;
            //         if(sum == num){ count++; break; }
            //         if(sum > num) break;
            //       }
            //   }
            //
            // This counts ordered pairs (i, o = num-i) where both are prime, by testing whether
            // num-i is prime. Then later we convert the ordered count into unordered count.
            for i in 2..=max_idx {
                if !composite[i] {
                    // Inner loop: iterate o from 2..=max_idx increasing;
                    // since o increases, when i+o > num we can break early.
                    let mut o = 2usize;
                    while o <= max_idx {
                        if !composite[o] {
                            let sum = (i + o) as i32;
                            if sum == num {
                                count += 1;
                                break;
                            } else if sum > num {
                                break;
                            }
                        }
                        o += 1;
                    }
                }
            }
        }

        // Convert ordered count to unordered:
        // The C code printed: count%2 ? (count/2+1) : (count/2)
        // Which is equivalent to ceil(count/2). (Integer arithmetic)
        let unordered = if count % 2 != 0 {
            count / 2 + 1
        } else {
            count / 2
        };

        output.push_str(&format!("{}\n", unordered));
    }

    // Print all collected outputs at once (preserves ordering and formatting).
    print!("{}", output);
}
