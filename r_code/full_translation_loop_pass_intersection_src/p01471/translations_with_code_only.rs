use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Item {
    w: i32,
    v: i32,
    r: f64,
}

fn zero(a: f64) -> bool {
    a.abs() < 1e-7
}

// Emulates the C comparator used by qsort:
// int cmp(T *a, T *b) { if (fabs(b->r - a->r)<1e-8) return 0; if (a->r <= b->r) return 1; return -1; }
fn cmp_items(a: &Item, b: &Item) -> Ordering {
    if (b.r - a.r).abs() < 1e-8 {
        Ordering::Equal
    } else if a.r <= b.r {
        // return 1 => a > b (so b comes before a): descending by r
        Ordering::Greater
    } else {
        // return -1 => a < b
        Ordering::Less
    }
}

// Fast-ish integer input matching the C in() behavior for well-formed inputs.
// Note: The original C code does not skip leading whitespace; it assumes the next
// character is either '-' or a digit. We preserve that assumption.
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
                // Mirror the C transformations exactly.
                w = -w;
                v = -v;
                ans -= v as f64;
                w_cap += w as f64;
            }
            // C does (double)v/w; if w==0 this yields +/-inf or NaN.
            // Rust f64 division matches IEEE-754 similarly.
            let r = (v as f64) / (w as f64);
            items.push(Item { w, v, r });
        }
    }

    // qsort with the given comparator; Rust's sort is stable, but stability is not required.
    items.sort_by(cmp_items);

    for it in &items {
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

    // C prints with "%lf\n" (default 6 digits after decimal).
    println!("{:.6}", ans);
}
