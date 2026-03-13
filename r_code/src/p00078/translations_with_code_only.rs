use std::io::{self, Read};

fn main() {
    // Read all input (the C program used scanf in a loop; reading the whole stdin
    // up front is fine for the same effect for non-interactive inputs).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse all whitespace-separated integers.
    let mut nums = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());

    // The original C program used a fixed 16x16 array:
    // int magic[16][16] = {0};
    // We recreate that here. The original program exhibits undefined behavior
    // if n <= 0 or n > 16; this translation assumes input values are the same
    // kind of sensible values the C program would be expected to get (positive
    // n not exceeding 16). If inputs violate that, behavior in this Rust
    // program may panic (index out of bounds) similar to how the original C
    // program would have undefined behaviour.
    const MAX: usize = 16;

    while let Some(n) = nums.next() {
        if n == 0 {
            break;
        }

        // Preserve C's int semantics by using i32 for arithmetic where needed.
        let n_i32 = n;
        // Convert to usize for indexing; this assumes n > 0 and n as usize <= MAX.
        let n_usize = n_i32 as usize;

        let mut magic = [[0i32; MAX]; MAX];

        // hn = n / 2  (C integer division truncates toward zero; same as Rust i32/2)
        let hn = n_i32 / 2;
        let mut x = hn;
        let mut y = hn;
        y += 1;

        let mut number: i32 = 1;
        // loop while number <= n * n
        // all arithmetic here mirrors the C code (i32 and using rem_euclid to
        // emulate the non-negative modulo behavior from the original code).
        while number <= n_i32 * n_i32 {
            // Convert indices to usize after reducing them into 0..n-1 via rem_euclid.
            let yi = y.rem_euclid(n_i32) as usize;
            let xi = x.rem_euclid(n_i32) as usize;

            // Access exactly as the C code: magic[y][x]
            if magic[yi][xi] != 0 {
                // y = (y + 1 + n) % n;
                // x = (x - 1 + n) % n;
                y = (y + 1 + n_i32).rem_euclid(n_i32);
                x = (x - 1 + n_i32).rem_euclid(n_i32);
                continue;
            }
            magic[yi][xi] = number;
            // y = (y + 1) % n;
            // x = (x + 1) % n;
            y = (y + 1).rem_euclid(n_i32);
            x = (x + 1).rem_euclid(n_i32);
            number += 1;
        }

        // Print with the same formatting as printf("%4d", magic[i][j]);
        for i in 0..n_usize {
            for j in 0..n_usize {
                // Right-aligned in a field of width 4.
                print!("{:4}", magic[i][j]);
            }
            // puts("") in C prints a newline.
            println!();
        }
    }
}
