use std::io::{self, BufRead};

const MAX_LENGTH: usize = 14;

struct Solver {
    p: [u8; MAX_LENGTH],
    result: [u8; MAX_LENGTH],
    used: [bool; MAX_LENGTH],
    ans_buf: Vec<u8>,
    top: usize,
    solved: bool,
}

impl Solver {
    fn new() -> Self {
        Self {
            p: [0; MAX_LENGTH],
            result: [0; MAX_LENGTH],
            used: [false; MAX_LENGTH],
            ans_buf: Vec::new(),
            top: 0,
            solved: false,
        }
    }

    fn push(&mut self, c: u8) {
        self.ans_buf.push(c);
        self.top += 1;
    }

    fn bi_ch(&self) -> bool {
        self.result[0] != self.result[1]
    }

    fn tri_ch(&self, n: usize) -> bool {
        if self.result[n - 2] == self.result[n - 1] && self.result[n - 1] == self.result[n] {
            return false;
        }
        if self.result[n - 2] == self.result[n - 1] + 1 && self.result[n - 1] == self.result[n] + 1 {
            return false;
        }
        true
    }

    fn calc(&mut self, n: usize) {
        if self.solved {
            return;
        }
        if n == 2 && !self.bi_ch() {
            return;
        }
        if n == 5 && !self.tri_ch(4) {
            return;
        }
        if n == 8 && !self.tri_ch(7) {
            return;
        }
        if n == 11 && !self.tri_ch(10) {
            return;
        }
        if n < MAX_LENGTH {
            for i in 0..MAX_LENGTH {
                if self.used[i] {
                    continue;
                }
                self.used[i] = true;
                self.result[n] = self.p[i];
                self.calc(n + 1);
                self.used[i] = false;
            }
        } else {
            if !self.tri_ch(13) {
                self.solved = true;
                self.push(self.p[13]);
            }
        }
    }

    fn dup_ch(&self, n: u8) -> bool {
        let count = self.p.iter().take(13).filter(|&&x| x == n).count();
        count >= 4
    }

    fn init(&mut self) {
        for i in 0..13 {
            self.p[i] -= b'0';
            self.used[i] = false;
        }
        self.top = 0;
        self.p[13] = 0;
    }

    fn solve(&mut self) {
        for i in 1..=9 {
            if self.dup_ch(i) {
                continue;
            }
            self.p[13] = i;
            self.solved = false;
            self.calc(0);
        }
    }

    fn print_result(&self) {
        if self.top == 0 {
            println!("0");
        } else {
            print!("{}", self.ans_buf[0] + b'0' as char);
            for &c in &self.ans_buf[1..] {
                print!(" {}", c + b'0' as char);
            }
            println!();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let input = line.unwrap();
        let mut solver = Solver::new();
        solver.init();
        for (i, c) in input.bytes().enumerate() {
            if i < 13 {
                solver.p[i] = c;
            }
        }
        solver.solve();
        solver.print_result();
    }
}
