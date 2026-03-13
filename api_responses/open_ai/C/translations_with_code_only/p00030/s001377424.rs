// translated_from_c.rs
// Translation of the provided C program to Rust.
// Preserves behavior of the original C code (combinatorial counting using a global-like val array).
// Uses safe, idiomatic Rust.

use std::io::{self, Read};

fn get_way(val: &mut [bool; 10], x: usize, n: i32, sum: i32, s: i32) -> i32 {
    // Mirrors the C function:
    // int getWay(int x, int n, int sum) { ... }
    let mut ret: i32 = 0;

    if n == 0 {
        if sum == s {
            return 1;
        }
        return 0;
    }

    for i in x..10 {
        // C checks val[i] == 0; we use bool where false==0.
        // The pruning condition sum + i <= s is preserved.
        if !val[i] && sum + (i as i32) <= s {
            val[i] = true;
            ret += get_way(val, i, n - 1, sum + (i as i32), s);
            val[i] = false;
        }
    }

    ret
}

fn main() {
    // Read all stdin, split by whitespace into integers (i32), then process pairs (n, s).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let mut nums: Vec<i32> = Vec::new();
    for tok in input.split_whitespace() {
        if let Ok(v) = tok.parse::<i32>() {
            nums.push(v);
        }
        // If parsing fails, ignore token (similar to scanf ignoring non-matching input).
    }

    let mut out = String::new();
    let mut i = 0usize;
    while i + 1 < nums.len() {
        let n = nums[i];
        let s = nums[i + 1];
        i += 2;

        // Mimic the C loop: break when n == 0 && s == 0
        if n == 0 && s == 0 {
            break;
        }

        // val array cleared for each case
        let mut val = [false; 10];

        let ways = get_way(&mut val, 0usize, n, 0, s);
        out.push_str(&format!("{}\n", ways));
    }

    // Print all results. This matches the C program's behavior of printing each result followed by '\n'.
    print!("{}", out);
}
