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

    // Emulates the C in() behavior: reads an integer assuming the stream is well-formed.
    // It does not skip whitespace explicitly; it relies on the input format (as the C code does).
    fn in_i32(&mut self) -> i32 {
        if self.idx >= self.buf.len() {
            return 0;
        }
        let mut c = self.buf[self.idx];
        self.idx += 1;

        let mut n: i32 = 0;
        if c == b'-' {
            if self.idx >= self.buf.len() {
                return 0;
            }
            c = self.buf[self.idx];
            self.idx += 1;
            while c >= b'0' {
                // (c & 0xf) matches the C code's digit extraction.
                n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
                if self.idx >= self.buf.len() {
                    break;
                }
                c = self.buf[self.idx];
                self.idx += 1;
            }
            return n.wrapping_neg();
        }

        while c >= b'0' {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            if self.idx >= self.buf.len() {
                break;
            }
            c = self.buf[self.idx];
            self.idx += 1;
        }
        n
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    let mut x = [0i32; 102];
    let mut y = [0i32; 102];

    loop {
        let n = sc.in_i32();
        if n == 0 {
            break;
        }
        let n_usize = n as usize;

        for i in 0..n_usize {
            x[i] = sc.in_i32();
            y[i] = sc.in_i32();
        }
        x[n_usize] = x[0];
        y[n_usize] = y[0];

        let mut xmin = INF;
        let mut ymin = INF;
        let mut xmax = -INF;
        let mut ymax = -INF;

        for _ in 0..4 {
            let a = sc.in_i32();
            let b = sc.in_i32();
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

                let mut a = x[i];
                if a > xmax {
                    a = xmax;
                }
                if xmin > a {
                    a = xmin;
                }

                let mut b = y[i];
                if b > ymax {
                    b = ymax;
                }
                if ymin > b {
                    b = ymin;
                }

                let mut c = y[i + 1];
                if c > ymax {
                    c = ymax;
                }
                if ymin > c {
                    c = ymin;
                }

                win += (a as i64) * ((b - c) as i64);
            }
        }

        out.push_str(&(win - cur).to_string());
        out.push('\n');
    }

    print!("{out}");
}
