// AOJ 0334 Amidakuji
// 2019.8.11 bal4u

use std::io::{self, BufRead, Write};

fn gc() -> char {
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.bytes().next().unwrap().unwrap() as char
}

fn pc(c: char) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(c.to_string().as_bytes()).unwrap();
}

fn in_non_negative() -> usize {
    let mut n = 0;
    let mut c = gc();
    while c >= '0' {
        n = 10 * n + (c as usize - '0' as usize);
        c = gc();
    }
    n
}

fn out(n: usize) {
    let mut ob = Vec::new();
    let mut num = n;
    while num > 0 {
        ob.push((num % 10) as u8 + b'0');
        num /= 10;
    }
    ob.reverse();
    for &c in &ob {
        pc(c as char);
    }
    pc('\n');
}

fn outs(s: &str) {
    for c in s.chars() {
        pc(c);
    }
    pc('\n');
}

const MAX: usize = 1005;
let mut hi = [0; MAX];
let mut to = [[0; MAX]; MAX];
let mut seen = [0; MAX];
let mut match_arr = [-1; MAX];
let mut mark = 1;

fn bpm(u: usize) -> bool {
    let mut i = 0;
    while i < hi[u] {
        let v = to[u][i];
        let w = match_arr[v];
        if w < 0 || (seen[w] != mark && bpm(w as usize)) {
            match_arr[v] = u as isize;
            match_arr[u] = v as isize;
            return true;
        }
        i += 1;
    }
    false
}

fn bipartite_matching(m: usize, n: usize) -> usize {
    let mut max = 0;
    for u in 0..m {
        if match_arr[u] < 0 {
            mark += 1;
            if bpm(u) {
                max += 1;
            }
        }
    }
    max
}

fn main() {
    let n = in_non_negative() - 1;
    for i in 0..n {
        for j in 0..n {
            if gc() as u8 & 1 != 0 {
                to[j][hi[j]] = n + i;
                hi[j] += 1;
            }
            gc(); // consume the newline or space
        }
    }
    if bipartite_matching(n, n) < n {
        outs("no");
    } else {
        outs("yes");
        for i in 0..n {
            match_arr[match_arr[i] as usize] = -1;
            match_arr[i] = -1;
            mark += 1;
            bpm(i);
            hi[i] = 0;
            to[i][hi[i]] = match_arr[i] as usize;
            hi[i] += 1;
            out(match_arr[i] as usize - n + 1);
        }
    }
}
