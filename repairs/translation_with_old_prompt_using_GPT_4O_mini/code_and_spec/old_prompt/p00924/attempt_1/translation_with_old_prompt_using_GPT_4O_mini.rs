use std::io::{self, BufRead};

fn solve(bit: &mut Vec<char>, runlen: &mut Vec<i32>, len: usize, runlenlen: usize, start: char) -> i32 {
    if len == 1 {
        return 0;
    }
    if runlen[0] == 0 {
        return solve(bit, &mut runlen[1..].to_vec(), len, runlenlen - 1, if start == '1' { '0' } else { '1' });
    }
    if len == 2 {
        return if bit[0] == start { 0 } else { 1 };
    } else if bit[0] == start {
        runlen[0] -= 1;
        return solve(&mut bit[1..].to_vec(), runlen, len - 1, runlenlen, start);
    } else if start == '0' {
        if let Some(pos) = bit.iter().position(|&c| c == '0') {
            runlen[0] -= 1;
            bit[pos] = '1';
            return (pos as i32) + solve(&mut bit[1..].to_vec(), runlen, len - 1, runlenlen, start);
        }
    } else if start == '1' {
        if let Some(pos) = bit.iter().position(|&c| c == '1') {
            runlen[0] -= 1;
            bit[pos] = '0';
            return (pos as i32) + solve(&mut bit[1..].to_vec(), runlen, len - 1, runlenlen, start);
        }
    }
    0 // Fallback return, should not reach here
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut b: Vec<char> = Vec::with_capacity(n);
    let mut p: Vec<i32> = Vec::with_capacity(m);
    let mut c1 = 0;
    let mut ce = 0;

    for _ in 0..n {
        let char_line = lines.next().unwrap().unwrap();
        let char_value = char_line.chars().next().unwrap();
        b.push(char_value);
        if char_value == '1' {
            c1 += 1;
        }
    }

    for i in 0..m {
        let run_length: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        p.push(run_length);
        if i % 2 == 0 {
            ce += run_length;
        }
    }

    let mut ret = 0;
    if c1 * 2 == n {
        let mut b1 = b.clone();
        let mut p1 = p.clone();
        let ret1 = solve(&mut b, &mut p.clone(), n, m, '1');
        let ret2 = solve(&mut b1, &mut p1.clone(), n, m, '0');
        ret = ret1.min(ret2);
    } else if ce == c1 {
        ret = solve(&mut b, &mut p.clone(), n, m, '1');
    } else {
        ret = solve(&mut b, &mut p.clone(), n, m, '0');
    }

    println!("{}", ret);
}
