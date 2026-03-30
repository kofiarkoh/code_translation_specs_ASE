use std::io::{self, BufRead, Write};

const MAX: usize = 1005;

struct BipartiteMatcher {
    hi: [usize; MAX],
    to: [[usize; MAX]; MAX],
    seen: [usize; MAX],
    match_: [isize; MAX],
    mark: usize,
}

impl BipartiteMatcher {
    fn new() -> Self {
        BipartiteMatcher {
            hi: [0; MAX],
            to: [[0; MAX]; MAX],
            seen: [0; MAX],
            match_: [-1; MAX],
            mark: 1,
        }
    }

    fn bpm(&mut self, u: usize) -> bool {
        self.seen[u] = self.mark;
        for i in 0..self.hi[u] {
            let v = self.to[u][i];
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
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse::<usize>().unwrap() - 1;
    let mut matcher = BipartiteMatcher::new();

    for i in 0..=n {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            if c == '1' {
                matcher.to[j][matcher.hi[j]] = n + i;
                matcher.hi[j] += 1;
            }
        }
    }

    if matcher.bipartite_matching(n + 1, n + 1) < n + 1 {
        println!("no");
    } else {
        println!("yes");
        for i in 0..=n {
            matcher.match_[matcher.match_[i] as usize] = -1;
            matcher.match_[i] = -1;
            matcher.mark += 1;
            matcher.bpm(i);
            matcher.hi[i] = 0;
            matcher.to[i][matcher.hi[i]] = matcher.match_[i] as usize;
            matcher.hi[i] += 1;
            println!("{}", matcher.match_[i] - n);
        }
    }
}
