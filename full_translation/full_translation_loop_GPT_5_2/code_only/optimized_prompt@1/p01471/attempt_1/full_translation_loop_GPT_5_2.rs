use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Item {
    w: i32,
    v: i32,
    r: f64,
}

fn zero(a: f64) -> bool {
    a.abs() < 1e-7
}

// Emulates the C in() which assumes the next non-consumed byte is either '-' or a digit.
// It does not skip whitespace; this matches the original behavior on valid AOJ input.
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
    fn next_i32(&mut self) -> i32 {
        // No whitespace skipping to mirror the C code's behavior.
        let mut c = self.buf[self.idx];
        self.idx += 1;

        let mut n: i32 = 0;
        if c == b'-' {
            c = self.buf[self.idx];
            self.idx += 1;
            while c >= b'0' {
                n = n * 10 + ((c & 0x0f) as i32);
                c = self.buf[self.idx];
                self.idx += 1;
            }
            -n
        } else {
            while c >= b'0' {
                n = n * 10 + ((c & 0x0f) as i32);
                c = self.buf[self.idx];
                self.idx += 1;
            }
            n
        }
    }
}

fn main() {
    let mut sc = Scanner::new();

    let mut n = sc.next_i32();
    let mut w_cap: f64 = sc.next_i32() as f64;

    let mut items: Vec<Item> = Vec::new();
    items.reserve(100_002);

    let mut ans: f64 = 0.0;

    while n > 0 {
        n -= 1;
        let mut w = sc.next_i32();
        let mut v = sc.next_i32();

        if w >= 0 && v <= 0 {
            continue;
        }
        if w < 0 && v > 0 {
            ans += v as f64;
            w_cap -= w as f64; // subtracting negative increases capacity
        } else {
            if w < 0 {
                // Mirror C: w=-w, v=-v, ans -= v, W += w (after sign flip)
                w = -w;
                v = -v;
                ans -= v as f64;
                w_cap += w as f64;
            }
            // C does (double)v/w; if w==0 this yields +/-inf or NaN; keep IEEE behavior.
            let r = (v as f64) / (w as f64);
            items.push(Item { w, v, r });
        }
    }

    // C qsort with comparator:
    // if |b.r - a.r| < 1e-8 => equal
    // else if a.r <= b.r => return 1 (a after b) => descending by r
    items.sort_by(|a, b| {
        if (b.r - a.r).abs() < 1e-8 {
            Ordering::Equal
        } else if a.r <= b.r {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    for it in items.iter() {
        if zero(w_cap) {
            break;
        }
        let mut x = w_cap / (it.w as f64);
        if x > 1.0 {
            x = 1.0;
        }
        w_cap -= x * (it.w as f64);
        ans += x * (it.v as f64);
    }

    // Match C printf("%lf\n", ans);
    println!("{:.6}", ans);
}
