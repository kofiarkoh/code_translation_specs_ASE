// AOJ 0334 Amidakuji
// 2019.8.11 bal4u

use std::io::{self, BufRead, Write};

const MAX: usize = 1005;

static mut HI: [usize; MAX] = [0; MAX];
static mut TO: [[usize; MAX]; MAX] = [[0; MAX]; MAX];
static mut SEEN: [usize; MAX] = [0; MAX];
static mut MATCH: [isize; MAX] = [-1; MAX];
static mut MARK: usize = 1;

fn in_non_negative() -> usize {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

fn out(n: isize) {
    let mut buffer = Vec::new();
    let mut num = n;
    if num == 0 {
        buffer.push(b'0');
    } else {
        while num > 0 {
            buffer.push((num % 10) as u8 + b'0');
            num /= 10;
        }
        buffer.reverse();
    }
    io::stdout().write_all(&buffer).unwrap();
    io::stdout().write_all(b"\n").unwrap();
}

fn outs(s: &str) {
    io::stdout().write_all(s.as_bytes()).unwrap();
    io::stdout().write_all(b"\n").unwrap();
}

fn bpm(u: usize) -> bool {
    unsafe {
        SEEN[u] = MARK;
        for i in 0..HI[u] {
            let v = TO[u][i];
            let w = MATCH[v];
            if w < 0 || (SEEN[w as usize] != MARK && bpm(w as usize)) {
                MATCH[v] = u as isize;
                MATCH[u] = v as isize;
                return true;
            }
        }
    }
    false
}

fn bipartite_matching(m: usize) -> usize {
    unsafe {
        MATCH.fill(-1);
        let mut max = 0;
        for u in 0..m {
            if MATCH[u] < 0 {
                MARK += 1;
                if bpm(u) {
                    max += 1;
                }
            }
        }
        max
    }
}

fn main() {
    let n = in_non_negative();
    unsafe {
        for i in 0..n {
            let line = io::stdin().lock().lines().next().unwrap().unwrap();
            let values: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            for j in 0..n {
                if values[j] & 1 != 0 {
                    TO[j][HI[j]] = n + i;
                    HI[j] += 1;
                }
            }
        }
        if bipartite_matching(n) < n {
            outs("no");
        } else {
            outs("yes");
            for i in 0..n {
                if MATCH[i] >= 0 {
                    out(MATCH[i] - n as isize + 1);
                }
            }
        }
    }
}
