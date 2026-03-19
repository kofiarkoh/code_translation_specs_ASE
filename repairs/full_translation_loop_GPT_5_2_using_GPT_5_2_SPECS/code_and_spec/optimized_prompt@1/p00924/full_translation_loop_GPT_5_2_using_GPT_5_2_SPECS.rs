use std::io::{self, Read};

fn solve(bit: &mut [u8], runlen: &mut [i32], len: usize, start: u8) -> i32 {
    if len == 1 {
        return 0;
    }
    if runlen[0] == 0 {
        // Move to next run; toggle start bit.
        let next_start = if start == b'1' { b'0' } else { b'1' };
        return solve(bit, &mut runlen[1..], len, next_start);
    }
    if len == 2 {
        return if bit[0] == start { 0 } else { 1 };
    }

    if bit[0] == start {
        runlen[0] -= 1;
        return solve(&mut bit[1..], runlen, len - 1, start);
    }

    // Find next occurrence of required bit at or to the right, swap it into position 0
    // by "moving" it left; cost is the distance.
    let target = start;
    let pos = bit
        .iter()
        .position(|&c| c == target)
        .unwrap(); // Undefined behavior in spec for infeasible inputs; match C's lack of checks.

    runlen[0] -= 1;
    // Mark the used bit as flipped so it can't be reused later (models consuming that bit).
    bit[pos] = if target == b'0' { b'1' } else { b'0' };

    (pos as i32) + solve(&mut bit[1..], runlen, len - 1, start)
}

fn main() {
    // Read all input; C code uses scanf with whitespace, so tokenization is fine.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut b: Vec<u8> = Vec::with_capacity(n);
    let mut c1: i32 = 0;
    for _ in 0..n {
        let tok = it.next().unwrap();
        let ch = tok.as_bytes()[0];
        b.push(ch);
        if ch == b'1' {
            c1 += 1;
        }
    }

    let mut p: Vec<i32> = Vec::with_capacity(m);
    let mut ce: i32 = 0;
    for i in 0..m {
        let v: i32 = it.next().unwrap().parse().unwrap();
        p.push(v);
        if (i & 1) == 0 {
            ce += v;
        }
    }

    let ret: i32;
    if c1 * 2 == n as i32 {
        // Try both starts and take min.
        let mut b1 = b.clone();
        let mut p1 = p.clone();
        let mut b2 = b;
        let mut p2 = p;

        let ret1 = solve(&mut b2[..], &mut p2[..], n, b'1');
        let ret2 = solve(&mut b1[..], &mut p1[..], n, b'0');
        ret = ret1.min(ret2);
    } else if ce == c1 {
        let mut b2 = b;
        let mut p2 = p;
        ret = solve(&mut b2[..], &mut p2[..], n, b'1');
    } else {
        let mut b2 = b;
        let mut p2 = p;
        ret = solve(&mut b2[..], &mut p2[..], n, b'0');
    }

    print!("{ret}\n");
}
