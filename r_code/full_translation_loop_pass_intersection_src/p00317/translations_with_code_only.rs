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
    fn gc(&mut self) -> u8 {
        // The original uses getchar_unlocked(); on EOF it would return EOF (-1).
        // AOJ inputs are well-formed; we still guard by returning 0 on EOF.
        if self.idx >= self.buf.len() {
            0
        } else {
            let c = self.buf[self.idx];
            self.idx += 1;
            c
        }
    }
    fn in_int(&mut self) -> i32 {
        // Matches C in(): reads digits until a non-digit (< '0') is seen.
        // Assumes first char is a digit (as in the original).
        let mut n: i32 = 0;
        let mut c = self.gc();
        loop {
            n = n * 10 + ((c & 0x0f) as i32);
            c = self.gc();
            if c < b'0' {
                break;
            }
        }
        n
    }
    fn ins(&mut self, s: &mut Vec<u8>, r: &mut Vec<u8>) -> usize {
        // Reads a token: do *s = gc(); while (*s++ > ' ');
        // Then terminates with 0 and writes reverse into r, also 0-terminated.
        s.clear();
        r.clear();

        // do *s = gc();
        let mut c = self.gc();
        s.push(c);

        // while (*s++ > ' ');
        while *s.last().unwrap() > b' ' {
            c = self.gc();
            s.push(c);
        }

        // *--s = 0;  (replace the delimiter with NUL)
        s.pop();
        s.push(0);

        let n = s.len() - 1; // length excluding NUL

        // while (--s >= ss) *r++ = *s;
        // Copy reversed (excluding NUL)
        for &ch in s[..n].iter().rev() {
            r.push(ch);
        }
        r.push(0);

        n
    }
}

fn cmp_bytes(a: &[u8], b: &[u8]) -> Ordering {
    // strcmp semantics on NUL-terminated strings.
    let mut i = 0usize;
    loop {
        let ac = *a.get(i).unwrap_or(&0);
        let bc = *b.get(i).unwrap_or(&0);
        if ac != bc {
            return ac.cmp(&bc);
        }
        if ac == 0 {
            return Ordering::Equal;
        }
        i += 1;
    }
}

fn strncmp_bytes(a: &[u8], b: &[u8], n: usize) -> Ordering {
    // strncmp semantics for first n bytes, stopping early on NUL.
    for i in 0..n {
        let ac = *a.get(i).unwrap_or(&0);
        let bc = *b.get(i).unwrap_or(&0);
        if ac != bc {
            return ac.cmp(&bc);
        }
        if ac == 0 {
            return Ordering::Equal;
        }
    }
    Ordering::Equal
}

// Binary search as in C code.
// idx: 0 => use w, 1 => use r
// part: 0 => exact match (strcmp), 1 => prefix count (strncmp with length n)
fn search(s: &[u8], n: usize, idx: usize, part: i32, w: &[Vec<u8>], r: &[Vec<u8>]) -> i32 {
    let arr = if idx != 0 { r } else { w };
    let n_total = arr.len() as i32;

    let mut low: i32 = 0;
    let mut high: i32 = n_total;

    if part != 0 {
        // upper bound for <=
        while low < high {
            let mid = (low + high) >> 1;
            let ord = strncmp_bytes(&arr[mid as usize], s, n);
            if ord != Ordering::Greater {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        let k = low - 1;

        // lower bound for <
        low = 0;
        high = n_total;
        while low < high {
            let mid = (low + high) >> 1;
            let ord = strncmp_bytes(&arr[mid as usize], s, n);
            if ord == Ordering::Less {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        return k - low + 1;
    } else {
        while low < high {
            let mid = (low + high) >> 1;
            let ord = cmp_bytes(&arr[mid as usize], s);
            if ord == Ordering::Equal {
                return 1;
            }
            if ord == Ordering::Less {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
    }
    0
}

fn main() {
    let mut sc = Scanner::new();

    let n = sc.in_int() as usize;
    let mut m = sc.in_int() as i32;

    let mut w: Vec<Vec<u8>> = Vec::with_capacity(n);
    let mut r: Vec<Vec<u8>> = Vec::with_capacity(n);

    let mut one: i32 = 0;
    for _ in 0..n {
        let mut s = Vec::<u8>::new();
        let mut rev = Vec::<u8>::new();
        let len = sc.ins(&mut s, &mut rev);
        if len == 1 {
            one += 1;
        }
        w.push(s);
        r.push(rev);
    }

    w.sort_by(|a, b| cmp_bytes(a, b));
    r.sort_by(|a, b| cmp_bytes(a, b));

    let mut out = String::new();

    while m > 0 {
        m -= 1;

        let mut slate: Vec<u8> = Vec::new();
        let mut res: Vec<u8> = Vec::new();
        let mut nlen = sc.ins(&mut slate, &mut res); // length excluding NUL

        // find first '?'
        let mut kq: usize = 0;
        while kq < nlen {
            if slate[kq] == b'?' {
                break;
            }
            kq += 1;
        }

        let mut part: i32 = 0;
        let mut f_res: usize = 0;
        let mut use_res_as_s = false;

        // In C, s is a pointer to either slate or res.
        if !res.is_empty() && res[0] == b'*' {
            // slate[--n] = 0, part = 1;
            nlen -= 1;
            if slate.len() > nlen {
                slate[nlen] = 0;
            }
            part = 1;
            if kq >= nlen {
                // k = -1
                kq = usize::MAX;
            }
        } else if !slate.is_empty() && slate[0] == b'*' {
            // res[--n] = 0, part = 1, s = res, f_res = 1;
            nlen -= 1;
            if res.len() > nlen {
                res[nlen] = 0;
            }
            part = 1;
            use_res_as_s = true;
            f_res = 1;

            // if (k >= n+1) k = -1; else k = n - k;
            if kq >= nlen + 1 {
                kq = usize::MAX;
            } else {
                kq = nlen - kq;
            }
        } else if kq >= nlen {
            kq = usize::MAX;
        }

        let s_vec: &mut Vec<u8> = if use_res_as_s { &mut res } else { &mut slate };

        let ans: i32;
        if kq != usize::MAX {
            if nlen == 1 {
                ans = if part != 0 { n as i32 } else { one };
            } else {
                let mut sum: i32 = 0;
                for ch in b'a'..=b'z' {
                    // s[k] = i;
                    if s_vec.len() <= kq {
                        // Should not happen for well-formed input; keep behavior safe.
                        continue;
                    }
                    s_vec[kq] = ch;
                    sum += search(s_vec, nlen, f_res, part, &w, &r);
                }
                ans = sum;
            }
        } else {
            ans = search(s_vec, nlen, f_res, part, &w, &r);
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
