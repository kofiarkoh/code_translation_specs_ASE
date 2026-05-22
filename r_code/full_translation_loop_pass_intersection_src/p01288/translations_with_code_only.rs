use std::io::{self, Read};

struct Scanner {
    b: Vec<u8>,
    i: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        Self { b: s.into_bytes(), i: 0 }
    }
    #[inline]
    fn skip_ws(&mut self) {
        while self.i < self.b.len() && self.b[self.i].is_ascii_whitespace() {
            self.i += 1;
        }
    }
    #[inline]
    fn next_i32(&mut self) -> Option<i32> {
        self.skip_ws();
        if self.i >= self.b.len() {
            return None;
        }
        let mut sign = 1i32;
        if self.b[self.i] == b'-' {
            sign = -1;
            self.i += 1;
        }
        let mut val: i32 = 0;
        let mut any = false;
        while self.i < self.b.len() {
            let c = self.b[self.i];
            if c.is_ascii_digit() {
                any = true;
                // C int overflow is UB; inputs are expected to fit. Use wrapping to avoid panic.
                val = val.wrapping_mul(10).wrapping_add((c - b'0') as i32);
                self.i += 1;
            } else {
                break;
            }
        }
        if any { Some(val.wrapping_mul(sign)) } else { None }
    }
    #[inline]
    fn next_non_ws_byte(&mut self) -> Option<u8> {
        // The original C uses scanf("%c %d\n", ...). In practice, the command is a non-whitespace
        // character; reading raw %c can accidentally consume '\n' and break parsing, causing
        // non-termination. We therefore read the next non-whitespace byte, matching intended input.
        self.skip_ws();
        if self.i >= self.b.len() {
            None
        } else {
            let c = self.b[self.i];
            self.i += 1;
            Some(c)
        }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let n = match sc.next_i32() {
            Some(x) => x,
            None => break,
        };
        let qn = match sc.next_i32() {
            Some(x) => x,
            None => break,
        };
        if n == 0 {
            break;
        }
        let n = n as usize;
        let qn = qn as usize;

        // Match C globals' capacity (100005) but size to needed range; P is accessed up to index N.
        let mut p = vec![0i32; n + 1 + 1];
        let mut m = vec![0i32; n + 1 + 1];
        let mut q = vec![0u8; qn];
        let mut v = vec![0i32; qn];

        // for(i=r=0;i<N;P[i]=i,M[i]=0,i++);
        for i in 0..n {
            p[i] = i as i32;
            m[i] = 0;
        }
        let mut r: i64 = 0;

        // M[1]=1;
        if n + 1 >= 1 {
            m[1] = 1;
        }

        // for(i=2;i<=N;P[i]=t,i++) scanf("%d\n",&t);
        for i in 2..=n {
            let t = sc.next_i32().unwrap_or(0);
            p[i] = t;
        }

        // Read queries; count M occurrences.
        for i in 0..qn {
            let c = sc.next_non_ws_byte().unwrap_or(0);
            let val = sc.next_i32().unwrap_or(0);
            q[i] = c;
            v[i] = val;
            if c == b'M' {
                let vi = val as usize;
                if vi < m.len() {
                    m[vi] = m[vi].wrapping_add(1);
                }
            }
        }

        // Process in reverse.
        for i in (0..qn).rev() {
            if q[i] == b'M' {
                let vi = v[i] as usize;
                if vi < m.len() {
                    m[vi] = m[vi].wrapping_sub(1);
                }
            } else {
                let mut t = v[i] as usize;
                // for(t=v[i];!M[t];) t=P[t];
                // Assume valid inputs ensure termination (as in the C program).
                while m[t] == 0 {
                    t = p[t] as usize;
                }
                r += t as i64;
            }
        }

        out.push_str(&format!("{r}\n"));
    }

    print!("{out}");
}
