use std::io::{self, Read};

fn toggle_bit(b: u8) -> u8 {
    if b == b'0' { b'1' } else { b'0' }
}

fn compute_cost(mut s: Vec<u8>, p: &[i64], start_bit: u8) -> i64 {
    let mut i: usize = 0; // suffix start index
    let mut j: usize = 0; // run index
    let mut need: u8 = start_bit;

    // Initialize current run length, skipping leading zero-length runs (toggling each skip).
    let mut run: i64 = if !p.is_empty() { p[0] } else { 0 };
    while run == 0 {
        j += 1;
        need = toggle_bit(need);
        run = p[j]; // undefined if j out of bounds; matches spec's lack of checks
    }

    let mut cost: i64 = 0;

    loop {
        let rem = s.len() - i;

        // Termination behavior
        if rem == 1 {
            return cost;
        }
        if rem == 2 {
            if s[i] == need {
                return cost;
            } else {
                return cost + 1;
            }
        }

        // Skip zero-length runs without consuming a character.
        if run == 0 {
            j += 1;
            need = toggle_bit(need);
            run = p[j]; // undefined if j out of bounds
            continue;
        }

        // Consume current position.
        if s[i] == need {
            run -= 1;
            i += 1;
            continue;
        }

        // Mismatch case: find first occurrence of required bit in current suffix.
        let mut pos: usize = 0;
        while s[i + pos] != need {
            pos += 1;
            // If not found, behavior is undefined per spec (would run past end).
        }

        cost += pos as i64;
        run -= 1;

        // Mutate in place by flipping the found character to the opposite bit.
        let idx = i + pos;
        s[idx] = toggle_bit(s[idx]);

        // Advance suffix pointer by one (discard current position).
        i += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut s: Vec<u8> = Vec::with_capacity(n);
    let mut c1: i64 = 0;
    for _ in 0..n {
        let tok = it.next().unwrap();
        let b = tok.as_bytes()[0];
        if b == b'1' {
            c1 += 1;
        }
        s.push(b);
    }

    let mut p: Vec<i64> = Vec::with_capacity(m);
    let mut ce: i64 = 0;
    for idx in 0..m {
        let v: i64 = it.next().unwrap().parse().unwrap();
        if idx % 2 == 0 {
            ce += v;
        }
        p.push(v);
    }

    let n_i64 = n as i64;

    let ans = if c1 * 2 == n_i64 {
        let a = compute_cost(s.clone(), &p, b'1');
        let b = compute_cost(s, &p, b'0');
        if a < b { a } else { b }
    } else {
        let start = if ce == c1 { b'1' } else { b'0' };
        compute_cost(s, &p, start)
    };

    println!("{}", ans);
}
