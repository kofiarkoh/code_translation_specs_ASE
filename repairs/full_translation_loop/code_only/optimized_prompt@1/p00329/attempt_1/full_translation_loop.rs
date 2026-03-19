// AOJ 0334 Amidakuji
// 2019.8.11 bal4u

use std::io::{self, BufRead, Write};

const MAX: usize = 1005;

fn in_non_negative() -> usize {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

fn out(n: isize) {
    let mut buffer = n.to_string();
    buffer.push('\n');
    print!("{}", buffer);
}

fn outs(s: &str) {
    let mut buffer = s.to_string();
    buffer.push('\n');
    print!("{}", buffer);
}

struct BipartiteMatcher {
    hi: [usize; MAX],
    to: [[isize; MAX]; MAX],
    seen: [usize; MAX],
    match_: [isize; MAX],
    mark: usize,
}

impl BipartiteMatcher {
    fn new() -> Self {
        BipartiteMatcher {
            hi: [0; MAX],
            to: [[-1; MAX]; MAX],
            seen: [0; MAX],
            match_: [-1; MAX],
            mark: 1,
        }
    }

    fn bpm(&mut self, u: usize) -> bool {
        self.seen[u] = self.mark;
        for i in 0..self.hi[u] {
            let v = self.to[u][i] as usize;
            let w = self.match_[v];
            if w < 0 || (self.seen[w as usize] != self.mark && self.bpm(w as usize)) {
                self.match_[v] = u as isize;
                self.match_[u] = v as isize;
                return true;
            }
        }
        false
    }

    fn bipartite_matching(&mut self, m: usize, n: usize) -> usize {
        self.match_.fill(-1);
        let mut max = 0;
        for u in 0..m {
            if self.match_[u] < 0 {
                self.mark += 1;
                if self.bpm(u) {
                    max += 1;
                }
            }
        }
        max
    }
}

fn main() {
    let mut matcher = BipartiteMatcher::new();
    let n = in_non_negative() - 1;

    for i in 0..n {
        for j in 0..n {
            if io::stdin().bytes().next().unwrap().unwrap() & 1 != 0 {
                matcher.to[j][matcher.hi[j]] = (n + i) as isize;
                matcher.hi[j] += 1;
            }
            // Consume the newline character
            let _ = io::stdin().bytes().next();
        }
    }

    if matcher.bipartite_matching(n, n) < n {
        outs("no");
    } else {
        outs("yes");
        for i in 0..n {
            matcher.match_[matcher.match_[i] as usize] = -1;
            matcher.match_[i] = -1;
            matcher.mark += 1;
            matcher.bpm(i);
            matcher.hi[i] = 0;
            matcher.to[i][matcher.hi[i]] = matcher.match_[i];
            matcher.hi[i] += 1;
            out(matcher.match_[i] - n + 1);
        }
    }
}
