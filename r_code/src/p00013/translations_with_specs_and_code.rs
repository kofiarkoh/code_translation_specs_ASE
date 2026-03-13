// translated_stack.rs
//
// Translation of the given C program according to the SPEC.
// Notes:
// - The SPEC requires that reading stops on EOF or on the first unsuccessful integer parse.
//   The original C code used `while(scanf("%d", &n) != EOF)` which would behave differently
//   (it would loop if scanf returned 0 on a non-integer token). Per instruction, we follow
//   the SPEC and stop on the first non-integer token or EOF.
// - This program preserves the intended semantics: non-zero integers are pushes, zero is pop.
// - Preconditions from the SPEC (no pop on empty, <= 100 pushes) are NOT checked here;
//   violating them will cause a panic (bounds check) which corresponds to undefined behavior
//   in the C program per the SPEC.

use std::io::Read;

fn main() {
    // Read entire stdin into a string. This models reading tokens until EOF.
    let mut input = String::new();
    // Ignore read errors; treat as end-of-input similar to C behavior.
    let _ = std::io::stdin().read_to_string(&mut input);

    // Fixed-capacity storage as in the C program: capacity 100.
    let mut car: [i32; 100] = [0; 100];
    let mut out: [i32; 100] = [0; 100];

    // Use i and k as signed integers to mirror the C code behavior.
    // Indexing will cast to usize; if preconditions are violated this will panic (bounds check).
    let mut i: i32 = 0; // stack pointer / size
    let mut k: i32 = 0; // number of recorded pops

    // Iterate over whitespace-separated tokens. On first token that fails to parse as i32,
    // stop processing (matches SPEC: unsuccessful scanf terminates reading).
    for token in input.split_whitespace() {
        match token.parse::<i32>() {
            Ok(n) => {
                if n == 0 {
                    // Pop: decrement i, then record car[i] into out[k].
                    i -= 1;
                    // Cast to usize for indexing; this will panic on underflow/overflow,
                    // which mirrors undefined behavior under SPEC precondition violations.
                    out[k as usize] = car[i as usize];
                    k += 1;
                } else {
                    // Push: store at car[i], then increment i.
                    car[i as usize] = n;
                    i += 1;
                }
            }
            Err(_) => {
                // Stop on first non-integer token (unsuccessful parse), per SPEC.
                break;
            }
        }
    }

    // Print recorded popped values in chronological order, one per line.
    for idx in 0..(k as usize) {
        println!("{}", out[idx]);
    }

    // Exit with status 0 implicitly.
}
