use std::io::{self, Read};

fn solve(bit: &mut [u8], runlen: &mut [i32], len: usize, runlenlen: usize, start: u8) -> i32 {
    if len == 1 {
        return 0;
    }
    if runlen[0] == 0 {
        let next_start = if start == b'1' { b'0' } else { b'1' };
        return solve(bit, &mut runlen[1..], len, runlenlen - 1, next_start);
    }
    if len == 2 {
        if bit[0] == start {
            return 0;
        } else {
            return 1;
        }
    } else if bit[0] == start {
        runlen[0] -= 1;
        return solve(&mut bit[1..], runlen, len - 1, runlenlen, start);
    } else if start == b'0' {
        // C uses strchr(bit,'0') and assumes it exists.
        let pos = bit
            .iter()
            .position(|&c| c == b'0')
            .expect("C code assumes a matching character exists") as i32;
        runlen[0] -= 1;
        bit[pos as usize] = b'1';
        return pos + solve(&mut bit[1..], runlen, len - 1, runlenlen, start);
    } else {
        // start == '1'
        let pos = bit
            .iter()
            .position(|&c| c == b'1')
            .expect("C code assumes a matching character exists") as i32;
        runlen[0] -= 1;
        bit[pos as usize] = b'0';
        return pos + solve(&mut bit[1..], runlen, len - 1, runlenlen, start);
    }
}

fn main() {
    // Read entire stdin; C scanf with " %d %d " and "%c " skips whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut b: Vec<u8> = vec![0; n];
    let mut c1: i32 = 0;
    for i in 0..n {
        let tok = it.next().unwrap();
        let ch = tok.as_bytes()[0];
        b[i] = ch;
        if ch == b'1' {
            c1 += 1;
        }
    }

    let mut p: Vec<i32> = vec![0; m];
    let mut ce: i32 = 0;
    for i in 0..m {
        let v: i32 = it.next().unwrap().parse().unwrap();
        p[i] = v;
        if (i & 1) == 0 {
            ce += v;
        }
    }

    let ret: i32;
    if c1 * 2 == n as i32 {
        // Mimic memcpy(b1,b,sizeof(b)) where b is a fixed 15-byte array in C.
        // Here we only have N bytes; copying N preserves the used portion.
        let mut b1 = b.clone();
        let mut p1 = p.clone();
        let ret1 = solve(&mut b, &mut p, n, m, b'1');
        let ret2 = solve(&mut b1, &mut p1, n, m, b'0');
        ret = ret1.min(ret2);
    } else if ce == c1 {
        ret = solve(&mut b, &mut p, n, m, b'1');
    } else {
        ret = solve(&mut b, &mut p, n, m, b'0');
    }

    print!("{ret}\n");
}
