use std::cmp::Ordering;
use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        Self { buf: input, idx: 0 }
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        // Mirrors C in(): assumes first char is a digit; stops when c < '0'.
        let mut n: u32 = 0;
        while self.idx < self.buf.len() && self.buf[self.idx] < b'0' {
            self.idx += 1;
        }
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c < b'0' {
                break;
            }
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as u32);
            self.idx += 1;
        }
        n
    }

    #[inline]
    fn ins(&mut self) -> (Vec<u8>, Vec<u8>, usize) {
        // Mirrors C ins(s, r):
        // - reads bytes until <= ' ' (space) encountered; that delimiter is consumed
        // - returns n = length of token
        // - r is reverse of token
        while self.idx < self.buf.len() && self.buf[self.idx] <= b' ' {
            self.idx += 1;
        }
        let start = self.idx;
        while self.idx < self.buf.len() && self.buf[self.idx] > b' ' {
            self.idx += 1;
        }
        let token = self.buf[start..self.idx].to_vec();
        let mut rev = token.clone();
        rev.reverse();
        (token, rev, token.len())
    }
}

#[inline]
fn cmp_lex(a: &[u8], b: &[u8]) -> Ordering {
    // strcmp-like ordering on bytes (C strings without embedded NUL in this problem).
    a.cmp(b)
}

#[inline]
fn strncmp(a: &[u8], b: &[u8], n: usize) -> Ordering {
    // C strncmp semantics for our byte slices:
    // compare up to n bytes; if one ends early, treat missing as '\0'.
    for i in 0..n {
        let ac = *a.get(i).unwrap_or(&0);
        let bc = *b.get(i).unwrap_or(&0);
        if ac != bc {
            return ac.cmp(&bc);
        }
    }
    Ordering::Equal
}

// Binary search as in C search()
fn search(sorted: &[Vec<u8>], s: &[u8], n: usize, part: bool) -> i32 {
    let n_total = sorted.len();
    if part {
        // upper bound for <= (strncmp <= 0), then -1
        let mut low = 0usize;
        let mut high = n_total;
        while low < high {
            let mid = (low + high) >> 1;
            if strncmp(&sorted[mid], s, n) != Ordering::Greater {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        let k = low as isize - 1;

        // lower bound for < (strncmp < 0)
        low = 0;
        high = n_total;
        while low < high {
            let mid = (low + high) >> 1;
            if strncmp(&sorted[mid], s, n) == Ordering::Less {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        let lo = low as isize;
        (k - lo + 1) as i32
    } else {
        let mut low = 0usize;
        let mut high = n_total;
        while low < high {
            let mid = (low + high) >> 1;
            match cmp_lex(&sorted[mid], s) {
                Ordering::Equal => return 1,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        0
    }
}

fn main() {
    let mut sc = Scanner::new();

    let n_words = sc.next_u32() as usize;
    let mut m = sc.next_u32() as usize;

    let mut w: Vec<Vec<u8>> = Vec::with_capacity(n_words);
    let mut r: Vec<Vec<u8>> = Vec::with_capacity(n_words);
    let mut one = 0i32;

    for _ in 0..n_words {
        let (tok, rev, len) = sc.ins();
        if len == 1 {
            one += 1;
        }
        w.push(tok);
        r.push(rev);
    }

    w.sort_by(|a, b| cmp_lex(a, b));
    r.sort_by(|a, b| cmp_lex(a, b));

    let mut out = String::new();

    while m > 0 {
        m -= 1;

        let (mut slate, mut res, mut n) = sc.ins();

        // find first '?'
        let mut k = 0usize;
        while k < n && slate[k] != b'?' {
            k += 1;
        }

        let mut part = false;
        let mut f_res = false;
        // s points to either slate or res; in Rust we track by a flag.
        let mut use_res_as_s = false;

        if !res.is_empty() && res[0] == b'*' {
            // slate[--n] = 0, part = 1;
            if n > 0 {
                n -= 1;
                slate.truncate(n);
            } else {
                slate.clear();
            }
            part = true;
            if k >= n {
                k = usize::MAX; // represent -1
            }
        } else if !slate.is_empty() && slate[0] == b'*' {
            // res[--n] = 0, part = 1, s = res, f_res = 1;
            if n > 0 {
                n -= 1;
                res.truncate(n);
            } else {
                res.clear();
            }
            part = true;
            use_res_as_s = true;
            f_res = true;
            if k >= n + 1 {
                k = usize::MAX; // -1
            } else {
                // k = n - k;
                k = n.wrapping_sub(k);
            }
        } else if k >= n {
            k = usize::MAX; // -1
        }

        let ans: i32;
        if k != usize::MAX {
            if n == 1 {
                ans = if part { n_words as i32 } else { one };
            } else {
                let mut sum = 0i32;
                for ch in b'a'..=b'z' {
                    if use_res_as_s {
                        if k < res.len() {
                            res[k] = ch;
                        }
                        sum += search(&r, &res, n, part);
                    } else {
                        if k < slate.len() {
                            slate[k] = ch;
                        }
                        sum += search(&w, &slate, n, part);
                    }
                }
                ans = sum;
            }
        } else {
            if use_res_as_s {
                ans = search(&r, &res, n, part);
            } else {
                ans = search(&w, &slate, n, part);
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
