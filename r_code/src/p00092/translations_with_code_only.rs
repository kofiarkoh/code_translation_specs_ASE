// translation.rs
//
// Direct translation of the given C program into Rust,
// preserving semantics (including corner cases) as specified.
//
// Notes on choices to preserve behavior:
// - Uses i32 for values that correspond to C `int` in the original.
// - Binary search and is_ok logic mirror the C code exactly (including
//   behavior for target == 0).
// - Input is read token-wise (like scanf "%d" / "%s") using split_whitespace.
// - The grid is stored as Vec<Vec<char>> for direct indexing by row/col.

use std::io::{self, Read};

fn is_ok(map_ruiseki: &Vec<Vec<i32>>, row: usize, target: i32, map_size: usize) -> bool {
    let mut now_renzoku: i32 = 0;
    for col in 0..map_size {
        if map_ruiseki[row][col] >= target {
            now_renzoku += 1;
        } else {
            now_renzoku = 0;
        }
        if now_renzoku >= target {
            return true;
        }
    }
    false
}

fn main() {
    // Read all stdin and split by whitespace (matches scanf behavior).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("read stdin");
    let mut toks = s.split_whitespace();

    // Helper to get next token as i32
    let mut next_i32 = || -> Option<i32> {
        toks.next().and_then(|t| t.parse::<i32>().ok())
    };

    loop {
        let map_size_i32 = match next_i32() {
            Some(v) => v,
            None => break, // no more input
        };
        if map_size_i32 == 0 {
            break;
        }
        let map_size_usize = map_size_i32 as usize;

        // Read map rows (tokens), each is a string with no whitespace.
        let mut map: Vec<Vec<char>> = Vec::with_capacity(map_size_usize);
        for _ in 0..map_size_usize {
            let row_token = toks.next().unwrap_or("");
            // In C code scanf("%s", map[i]) would read whatever token is present.
            // Preserve characters as they are.
            let row_chars: Vec<char> = row_token.chars().collect();
            map.push(row_chars);
        }

        // Build map_ruiseki: for each column, compute vertical consecutive '.' counts
        // up to current row (reset on '*').
        let mut map_ruiseki: Vec<Vec<i32>> = vec![vec![0; map_size_usize]; map_size_usize];
        for col in 0..map_size_usize {
            let mut now: i32 = 0;
            for row in 0..map_size_usize {
                // In C, map[j][i] is a char; compare to '*' exactly.
                if map[row][col] == '*' {
                    now = 0;
                } else {
                    now += 1;
                }
                map_ruiseki[row][col] = now;
            }
        }

        // Main logic: for each row, binary search the largest square size
        // whose bottom edge is at that row. Keep global maximum in result.
        let mut result: i32 = 0;
        for row in 0..map_size_usize {
            let mut left: i32 = result;
            let mut right: i32 = map_size_i32;
            while left <= right {
                let mid = (left + right) / 2;
                if is_ok(&map_ruiseki, row, mid, map_size_usize) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            if left - 1 > result {
                result = left - 1;
            }
        }

        println!("{}", result);
    }
}
