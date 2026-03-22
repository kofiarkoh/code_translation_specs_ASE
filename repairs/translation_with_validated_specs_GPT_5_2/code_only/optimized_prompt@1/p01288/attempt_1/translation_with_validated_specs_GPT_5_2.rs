use std::io::{self, Read};

fn main() {
    // Read entire stdin; we'll implement a small scanner that mimics scanf-style token parsing.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    fn skip_ws(bytes: &[u8], idx: &mut usize) {
        while *idx < bytes.len() && bytes[*idx].is_ascii_whitespace() {
            *idx += 1;
        }
    }

    fn read_i32(bytes: &[u8], idx: &mut usize) -> Option<i32> {
        skip_ws(bytes, idx);
        if *idx >= bytes.len() {
            return None;
        }
        let mut sign = 1i32;
        if bytes[*idx] == b'-' {
            sign = -1;
            *idx += 1;
        }
        let mut val: i32 = 0;
        let mut any = false;
        while *idx < bytes.len() {
            let c = bytes[*idx];
            if c.is_ascii_digit() {
                any = true;
                val = val.saturating_mul(10).saturating_add((c - b'0') as i32);
                *idx += 1;
            } else {
                break;
            }
        }
        if any { Some(val * sign) } else { None }
    }

    fn read_char_token(bytes: &[u8], idx: &mut usize) -> Option<u8> {
        // In the C code: scanf("%c %d\n", &q[i], &v[i]);
        // %c does NOT skip whitespace, but the format string begins with "%c " (no leading space),
        // so it will read the next byte even if it's '\n'. However, typical inputs place the
        // command at the start of a line; to preserve intended behavior robustly, we mimic
        // scanf's behavior: read the next byte as-is (if any), then the following " %d" will
        // skip whitespace before the integer.
        if *idx >= bytes.len() {
            return None;
        }
        let c = bytes[*idx];
        *idx += 1;
        Some(c)
    }

    let mut out = String::new();

    loop {
        let n = match read_i32(bytes, &mut idx) {
            Some(x) => x,
            None => break,
        };
        let qn = match read_i32(bytes, &mut idx) {
            Some(x) => x,
            None => break,
        };
        if n == 0 {
            break;
        }
        let n_usize = n as usize;
        let q_usize = qn as usize;

        // Arrays sized like the C globals (100005). We'll size to n+2 for safety.
        let mut p = vec![0i32; n_usize + 2];
        let mut m = vec![0i32; n_usize + 2];
        let mut v = vec![0i32; q_usize + 2];
        let mut qc = vec![0u8; q_usize + 2];

        // for(i=r=0;i<N;P[i]=i,M[i]=0,i++);
        for i in 0..n_usize {
            p[i] = i as i32;
            m[i] = 0;
        }
        let mut r: i64 = 0;

        // M[1]=1;
        if n_usize + 1 > 1 {
            m[1] = 1;
        }

        // for(i=2;i<=N;P[i]=t,i++) scanf("%d\n",&t);
        // Note: P[0..N-1] initialized; this fills P[2..=N] (inclusive), so P[N] is used.
        for i in 2..=n_usize {
            let t = read_i32(bytes, &mut idx).unwrap_or(0);
            p[i] = t;
        }

        // for(i=0;i<Q;i++) { scanf("%c %d\n",&q[i],&v[i]); if(q[i]=='M')M[v[i]]++; }
        for i in 0..q_usize {
            let c = match read_char_token(bytes, &mut idx) {
                Some(c) => c,
                None => 0,
            };
            qc[i] = c;
            let val = read_i32(bytes, &mut idx).unwrap_or(0);
            v[i] = val;
            if c == b'M' {
                let vi = val as usize;
                if vi < m.len() {
                    m[vi] = m[vi].wrapping_add(1);
                }
            }
        }

        // for(i=Q-1;i>=0;i--) { ... }
        for i in (0..q_usize).rev() {
            if qc[i] == b'M' {
                let vi = v[i] as usize;
                if vi < m.len() {
                    m[vi] = m[vi].wrapping_sub(1);
                }
            } else {
                let mut t = v[i] as usize;
                // for(t=v[i];!M[t];) t=P[t];
                // This assumes t stays within bounds; we follow the C behavior for valid inputs.
                while t < m.len() && m[t] == 0 {
                    t = p[t] as usize;
                }
                r += t as i64;
            }
        }

        out.push_str(&format!("{r}\n"));
    }

    print!("{out}");
}
