use std::io::{self, Write};

const MAX_LEN: usize = 14;
const ANS_BUF_LEN: usize = 9;

struct Solver {
    p: [i8; MAX_LEN],
    result: [i8; MAX_LEN],
    used: [bool; MAX_LEN],
    ans_buf: [i8; ANS_BUF_LEN],
    top: usize,
    solved: bool,
}

impl Solver {
    fn new() -> Self {
        Self {
            p: [0; MAX_LEN],
            result: [0; MAX_LEN],
            used: [false; MAX_LEN],
            ans_buf: [0; ANS_BUF_LEN],
            top: 0,
            solved: false,
        }
    }

    fn push(&mut self, c: i8) {
        self.ans_buf[self.top] = c;
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
        if n == 2 && self.bi_ch() {
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
        if n < MAX_LEN {
            for i in 0..MAX_LEN {
                if self.solved {
                    break;
                }
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

    fn dup_ch(&self, n: i8) -> bool {
        let count = self.p.iter().take(13).filter(|&&x| x == n).count();
        count >= 4
    }

    fn init(&mut self) {
        for i in 0..13 {
            self.p[i] -= '0' as i8;
            self.used[i] = false;
        }
        self.top = 0;
        self.p[13] = 0;
        self.used[13] = false;
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
            print!("{}", self.ans_buf[0] + '0');
            for i in 1..self.top {
                print!(" {}", self.ans_buf[i] + '0');
            }
            println!();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    while stdin.read_line(&mut input).unwrap() > 0 {
        let mut solver = Solver::new();
        solver.init();
        for (i, c) in input.trim().chars().enumerate() {
            if i < 13 {
                solver.p[i] = (c as i8) - ('0' as i8);
            }
        }
        solver.solve();
        solver.print_result();
        input.clear();
    }
}
