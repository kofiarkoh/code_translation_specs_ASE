use std::io::{self, Read};

const INF: i32 = 20002;

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), idx: 0 }
    }
    fn next_i32(&mut self) -> Option<i32> {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        if self.idx >= self.buf.len() {
            return None;
        }
        let mut sign = 1i32;
        if self.buf[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let mut n: i32 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if !(b'0'..=b'9').contains(&c) {
                break;
            }
            n = n * 10 + (c - b'0') as i32;
            self.idx += 1;
        }
        Some(sign * n)
    }
}

fn clamp(v: i32, lo: i32, hi: i32) -> i32 {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let n = match sc.next_i32() {
            Some(v) => v,
            None => break,
        };
        if n == 0 {
            break; // termination condition as in the C code: while (N = in())
        }
        // Spec says N should be positive; behavior for non-positive is not defined.
        let n_usize = n as usize;

        let mut x = vec![0i32; n_usize + 1];
        let mut y = vec![0i32; n_usize + 1];

        for i in 0..n_usize {
            x[i] = sc.next_i32().unwrap();
            y[i] = sc.next_i32().unwrap();
        }
        x[n_usize] = x[0];
        y[n_usize] = y[0];

        let mut xmin = INF;
        let mut ymin = INF;
        let mut xmax = -INF;
        let mut ymax = -INF;

        for _ in 0..4 {
            let a = sc.next_i32().unwrap();
            let b = sc.next_i32().unwrap();
            if a < xmin {
                xmin = a;
            }
            if a > xmax {
                xmax = a;
            }
            if b < ymin {
                ymin = b;
            }
            if b > ymax {
                ymax = b;
            }
        }

        let mut win: i64 = 0;
        let mut cur: i64 = 0;

        for i in 0..n_usize {
            if x[i] == x[i + 1] {
                cur += (x[i] as i64) * ((y[i] - y[i + 1]) as i64);

                let a = clamp(x[i], xmin, xmax);
                let b = clamp(y[i], ymin, ymax);
                let c = clamp(y[i + 1], ymin, ymax);

                win += (a as i64) * ((b - c) as i64);
            }
        }

        out.push_str(&(win - cur).to_string());
        out.push('\n');
    }

    print!("{out}");
}
