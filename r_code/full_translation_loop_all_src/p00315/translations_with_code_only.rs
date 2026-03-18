use std::io::{self, Read};

fn main() {
    // Read entire stdin, tokenize by ASCII whitespace (matches scanf behavior for %d and %s).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let c: i32 = it.next().unwrap().parse().unwrap();
    let n: usize = it.next().unwrap().parse().unwrap();

    // Store as bytes b'0'/'1' to mirror C char behavior.
    let mut a: Vec<Vec<u8>> = vec![vec![0u8; n]; n];
    for i in 0..n {
        let s = it.next().unwrap();
        let bytes = s.as_bytes();
        for j in 0..n {
            a[i][j] = bytes[j];
        }
    }

    // t is 0/1 like C's char used as small integer.
    let mut t: Vec<Vec<i32>> = vec![vec![0i32; n]; n];
    let mut total: i32 = 0;

    let check = |a: &Vec<Vec<u8>>, n: usize, x: usize, y: usize| -> i32 {
        if a[x][y] == a[n - 1 - x][y] && a[x][y] == a[x][n - 1 - y] {
            1
        } else {
            0
        }
    };

    for i in 0..n {
        for j in 0..n {
            let v = check(&a, n, i, j);
            t[i][j] = v;
            total += v;
        }
    }

    let solve = |total: i32, n: usize| -> i32 {
        if total == (n * n) as i32 { 1 } else { 0 }
    };

    let mut ans: i32 = solve(total, n);

    // Helper closures that mutate captured state.
    let mut update = |a: &Vec<Vec<u8>>, t: &mut Vec<Vec<i32>>, total: &mut i32, x: usize, y: usize| {
        *total -= t[x][y];
        t[x][y] = check(a, n, x, y);
        *total += t[x][y];
    };

    let mut change = |a: &mut Vec<Vec<u8>>, t: &mut Vec<Vec<i32>>, total: &mut i32, x: usize, y: usize| {
        a[x][y] = if a[x][y] == b'0' { b'1' } else { b'0' };

        // Update three positions exactly as in C (duplicates allowed).
        update(a, t, total, x, y);
        update(a, t, total, n - 1 - x, y);
        update(a, t, total, x, n - 1 - y);
    };

    // Process C-1 batches.
    for _ in 0..(c - 1) {
        let k: i32 = it.next().unwrap().parse().unwrap();
        for _ in 0..k {
            let mut x: usize = it.next().unwrap().parse::<usize>().unwrap();
            let mut y: usize = it.next().unwrap().parse::<usize>().unwrap();
            // Convert 1-based to 0-based as in C.
            x -= 1;
            y -= 1;
            change(&mut a, &mut t, &mut total, x, y);
        }
        ans += solve(total, n);
    }

    print!("{ans}\n");
}
