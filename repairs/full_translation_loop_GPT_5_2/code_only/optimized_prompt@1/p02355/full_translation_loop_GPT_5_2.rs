use std::io::{self, Read};

const MAX_N: usize = 100_003;

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

    // Mimics the C in(): reads a non-negative integer, assuming input is well-formed.
    // It does not skip non-digits explicitly; it starts from current position.
    fn in_i32(&mut self) -> i32 {
        let mut n: i32 = 0;
        // In the original C, it reads one char first, then loops while c >= '0'.
        // Here we assume the same input constraints (digits followed by whitespace/newline/EOF).
        if self.idx >= self.buf.len() {
            return 0;
        }
        let mut c = self.buf[self.idx];
        self.idx += 1;

        loop {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            if self.idx >= self.buf.len() {
                break;
            }
            c = self.buf[self.idx];
            self.idx += 1;
            if c < b'0' {
                break;
            }
        }
        n
    }
}

fn main() {
    let mut sc = Scanner::new();

    let n = sc.in_i32() as usize;
    let k = sc.in_i32() as usize;

    if n < k {
        print!("0\n");
        return;
    }

    let mut a = vec![0i32; n];
    for i in 0..n {
        a[i] = sc.in_i32();
    }

    // f is indexed by value t (1..=K). Keep size MAX_N to match C's fixed array.
    let mut f = vec![0i32; MAX_N];

    let mut s: usize = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;

    while s < k && j < n {
        let t = a[j];
        j += 1;
        if t > 0 && (t as usize) <= k {
            let idx = t as usize;
            if f[idx] == 0 {
                s += 1;
            }
            f[idx] += 1;
        }
    }

    if s < k {
        print!("0\n");
        return;
    }

    // Shrink from left
    loop {
        let t = a[i];
        let cond = if t > 0 && (t as usize) <= k {
            f[t as usize] > 1
        } else {
            true // (t > K) in C; also treat t<=0 as "not in 1..=K"
        };
        if !cond {
            break;
        }
        if t > 0 && (t as usize) <= k {
            f[t as usize] -= 1;
        }
        i += 1;
    }

    let mut ans: usize = j - i;

    while j < n {
        let t = a[j];
        j += 1;
        if t > 0 && (t as usize) <= k {
            f[t as usize] += 1;
        }

        loop {
            let t2 = a[i];
            let cond = if t2 > 0 && (t2 as usize) <= k {
                f[t2 as usize] > 1
            } else {
                true
            };
            if !cond {
                break;
            }
            if t2 > 0 && (t2 as usize) <= k {
                f[t2 as usize] -= 1;
            }
            i += 1;
        }

        let len = j - i;
        if len < ans {
            ans = len;
        }
    }

    print!("{}\n", ans);
}
