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
        if self.idx >= self.buf.len() {
            0
        } else {
            let c = self.buf[self.idx];
            self.idx += 1;
            c
        }
    }

    // Emulates the C in(): reads a non-negative integer, assuming first char is a digit.
    fn in_int(&mut self) -> i32 {
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

    // Emulates ins(s, r):
    // - reads a token (sequence of bytes > ' ') into s, NUL-terminated
    // - writes its reverse into r, NUL-terminated
    // - returns length (excluding NUL)
    fn ins(&mut self, s: &mut Vec<u8>, r: &mut Vec<u8>) -> usize {
        s.clear();
        r.clear();

        // do *s = gc(); while (*s++ > ' ');
        let mut c = self.gc();
        loop {
            s.push(c);
            if c <= b' ' {
                break;
            }
            c = self.gc();
        }
        // *--s = 0;  (replace delimiter with NUL)
        if let Some(last) = s.last_mut() {
            *last = 0;
        }
        let n = s.len().saturating_sub(1);

        // while (--s >= ss) *r++ = *s;  (reverse, excluding NUL)
        for &ch in s[..n].iter().rev() {
            r.push(ch);
        }
        r.push(0);
        n
    }
}

// strcmp on NUL-terminated byte strings
fn strcmp(a: &[u8], b: &[u8]) -> i32 {
    let mut i = 0usize;
    loop {
        let ac = *a.get(i).unwrap_or(&0);
        let bc = *b.get(i).unwrap_or(&0);
        if ac != bc {
            return (ac as i32) - (bc as i32);
        }
        if ac == 0 {
            return 0;
        }
        i += 1;
    }
}

// strncmp on NUL-terminated byte strings, comparing at most n bytes
fn strncmp(a: &[u8], b: &[u8], n: usize) -> i32 {
    for i in 0..n {
        let ac = *a.get(i).unwrap_or(&0);
        let bc = *b.get(i).unwrap_or(&0);
        if ac != bc {
            return (ac as i32) - (bc as i32);
        }
        if ac == 0 {
            return 0;
        }
    }
    0
}

// Binary search as in the C code.
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
            let cmp = strncmp(&arr[mid as usize], s, n);
            if cmp <= 0 {
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
            let cmp = strncmp(&arr[mid as usize], s, n);
            if cmp < 0 {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        k - low + 1
    } else {
        while low < high {
            let mid = (low + high) >> 1;
            let k = strcmp(&arr[mid as usize], s);
            if k == 0 {
                return 1;
            }
            if k < 0 {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        0
    }
}

fn main() {
    let mut sc = Scanner::new();

    let n = sc.in_int() as usize;
    let mut m = sc.in_int() as i32;

    let mut w: Vec<Vec<u8>> = Vec::with_capacity(n);
    let mut r: Vec<Vec<u8>> = Vec::with_capacity(n);

    let mut one = 0i32;

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

    // qsort with strcmp
    w.sort_by(|a, b| {
        let c = strcmp(a, b);
        if c < 0 {
            Ordering::Less
        } else if c > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    r.sort_by(|a, b| {
        let c = strcmp(a, b);
        if c < 0 {
            Ordering::Less
        } else if c > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    let mut out = String::new();

    let mut slate = Vec::<u8>::new();
    let mut res = Vec::<u8>::new();

    while m > 0 {
        m -= 1;

        let mut nlen = sc.ins(&mut slate, &mut res); // length excluding NUL

        // find first '?'
        let mut kpos = 0usize;
        while kpos < nlen {
            if slate[kpos] == b'?' {
                break;
            }
            kpos += 1;
        }

        let mut part = 0i32;
        let mut f_res = 0usize;
        let mut use_res_as_s = false;

        // s points to slate by default
        if !res.is_empty() && res[0] == b'*' {
            // slate[--n] = 0, part = 1;
            if nlen > 0 {
                nlen -= 1;
                if slate.len() > nlen {
                    slate[nlen] = 0;
                }
            }
            part = 1;
            if kpos >= nlen {
                kpos = usize::MAX; // represent -1
            }
        } else if !slate.is_empty() && slate[0] == b'*' {
            // res[--n] = 0, part = 1, s = res, f_res = 1;
            if nlen > 0 {
                nlen -= 1;
                if res.len() > nlen {
                    res[nlen] = 0;
                }
            }
            part = 1;
            use_res_as_s = true;
            f_res = 1;

            if kpos >= nlen + 1 {
                kpos = usize::MAX; // -1
            } else {
                // k = n - k;
                kpos = nlen - kpos;
            }
        } else if kpos >= nlen {
            kpos = usize::MAX; // -1
        }

        let s_vec: &mut Vec<u8> = if use_res_as_s { &mut res } else { &mut slate };

        let ans: i32;
        if kpos != usize::MAX {
            if nlen == 1 {
                ans = if part != 0 { n as i32 } else { one };
            } else {
                let mut sum = 0i32;
                for ch in b'a'..=b'z' {
                    if kpos < s_vec.len() {
                        s_vec[kpos] = ch;
                    }
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
