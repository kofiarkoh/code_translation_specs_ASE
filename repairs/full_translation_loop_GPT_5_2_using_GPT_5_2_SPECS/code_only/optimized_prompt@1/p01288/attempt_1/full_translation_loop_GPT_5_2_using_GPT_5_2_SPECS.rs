use std::io::{self, Read};

fn main() {
    // Read entire stdin; we'll parse similarly to scanf with whitespace handling.
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
        let mut sign: i32 = 1;
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
                // C's int overflow is UB; assume inputs fit in i32 as typical contest code.
                val = val * 10 + (c - b'0') as i32;
                *idx += 1;
            } else {
                break;
            }
        }
        if any { Some(val * sign) } else { None }
    }

    fn read_char(bytes: &[u8], idx: &mut usize) -> Option<u8> {
        // In the C code: scanf("%c %d\n", ...) does NOT skip leading whitespace for %c.
        // It will read the next byte as-is (often '\n' after previous reads).
        if *idx >= bytes.len() {
            return None;
        }
        let c = bytes[*idx];
        *idx += 1;
        Some(c)
    }

    let mut out = String::new();

    loop {
        let n_opt = read_i32(bytes, &mut idx);
        let q_opt = read_i32(bytes, &mut idx);
        if n_opt.is_none() || q_opt.is_none() {
            break;
        }
        let n = n_opt.unwrap() as usize;
        let qn = q_opt.unwrap() as usize;
        if n == 0 {
            break;
        }

        // Arrays sized like the C globals; we only use indices up to N.
        let mut p = vec![0i32; 100005];
        let mut m = vec![0i32; 100005];
        let mut v = vec![0i32; 100005];
        let mut q = vec![0u8; 100005];

        let mut r: i64 = 0;

        // for(i=r=0;i<N;P[i]=i,M[i]=0,i++);
        for i in 0..n {
            p[i] = i as i32;
            m[i] = 0;
        }

        // M[1]=1;
        if 1 < m.len() {
            m[1] = 1;
        }

        // for(i=2;i<=N;P[i]=t,i++) scanf("%d\n",&t);
        // Note: this writes P[N] as well (<= N), so ensure we have that index.
        for i in 2..=n {
            let t = read_i32(bytes, &mut idx).unwrap_or(0);
            p[i] = t;
        }

        // for(i=0;i<Q;i++) { scanf("%c %d\n",&q[i],&v[i]); if(q[i]=='M')M[v[i]]++; }
        for i in 0..qn {
            let ch = read_char(bytes, &mut idx).unwrap_or(0);
            q[i] = ch;
            let val = read_i32(bytes, &mut idx).unwrap_or(0);
            v[i] = val;
            if ch == b'M' {
                let vi = val as usize;
                if vi < m.len() {
                    m[vi] += 1;
                }
            }
        }

        // for(i=Q-1;i>=0;i--)
        // Use i64 to mirror C's signed int loop behavior.
        let mut i = qn as i64 - 1;
        while i >= 0 {
            let ii = i as usize;
            if q[ii] == b'M' {
                let vi = v[ii] as usize;
                if vi < m.len() {
                    m[vi] -= 1;
                }
            } else {
                // for(t=v[i];!M[t];) t=P[t];
                let mut t = v[ii] as usize;
                // The C code assumes indices are valid; we mirror that as best as possible.
                while t < m.len() && m[t] == 0 {
                    let next = p[t] as isize;
                    if next < 0 {
                        // Would be UB/out-of-bounds in C; stop to avoid panic.
                        break;
                    }
                    t = next as usize;
                }
                r += t as i64;
            }
            i -= 1;
        }

        out.push_str(&format!("{}\n", r));
    }

    print!("{}", out);
}
