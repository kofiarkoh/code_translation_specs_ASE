use std::io::{self, Read};

fn main() {
    // Read entire stdin (fast enough; preserves exact I/O formatting: only prints answers with '\n')
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx = 0usize;

    // Emulate the C in(): reads a non-negative integer, assuming input is well-formed.
    // C version does not skip non-digits; it starts from current char and accumulates until c < '0'.
    // In typical AOJ input, numbers are separated by whitespace/newlines, so we skip non-digits here
    // to be robust while preserving intended behavior.
    fn read_u32(bytes: &[u8], idx: &mut usize) -> usize {
        while *idx < bytes.len() && (bytes[*idx] < b'0' || bytes[*idx] > b'9') {
            *idx += 1;
        }
        let mut n: usize = 0;
        while *idx < bytes.len() {
            let c = bytes[*idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n * 10 + (c - b'0') as usize;
            *idx += 1;
        }
        n
    }

    let n = read_u32(bytes, &mut idx);

    // Arrays are sized n+1 (1-based indexing), matching the C code's usage.
    let mut p = vec![0usize; n + 1];
    let mut hi = vec![0usize; n + 1];
    let mut ans = vec![0usize; n + 1];

    for i in 1..=n {
        let a = read_u32(bytes, &mut idx);
        p[i] = a;
        if a <= n {
            hi[a] += 1;
        } else {
            // The C code would index out of bounds (UB) if a > n.
            // We keep behavior defined in Rust by allowing growth, but only if needed.
            // However, AOJ inputs should satisfy 1..=n.
            // If it happens, extend arrays to preserve indexing semantics.
            let new_len = a + 1;
            if hi.len() < new_len {
                hi.resize(new_len, 0);
                ans.resize(new_len, 0);
            }
            hi[a] += 1;
        }
    }

    // q[i] is an array of positions where value i appears.
    // In C: q[i] = malloc(hi[i]*sizeof(int)); then hi reset to 0 and used as fill count.
    let mut q: Vec<Vec<usize>> = vec![Vec::new(); hi.len()];
    for i in 1..hi.len() {
        q[i] = Vec::with_capacity(hi[i]);
    }

    // Reset hi to 0 for reuse as "current size" (like C memset).
    for v in hi.iter_mut() {
        *v = 0;
    }

    for i in 1..=n {
        let a = p[i];
        if a >= q.len() {
            q.resize_with(a + 1, Vec::new);
            hi.resize(a + 1, 0);
            ans.resize(a + 1, 0);
        }
        q[a].push(i);
        hi[a] += 1;
    }

    // Stack sp with capacity n (C uses sp[MAX]).
    let mut sp: Vec<usize> = Vec::with_capacity(n);

    // Two passes: for j=0..1, for i=1..n:
    // while (hi[i]) sp[top++] = q[i][--hi[i]];
    // if (!ans[i] && top) ans[i] = sp[--top];
    for _j in 0..2 {
        for i in 1..=n {
            // Pop all occurrences of i from q[i] in reverse insertion order.
            while hi[i] != 0 {
                hi[i] -= 1;
                let pos = q[i][hi[i]];
                sp.push(pos);
            }
            if ans[i] == 0 && !sp.is_empty() {
                ans[i] = sp.pop().unwrap();
            }
        }
    }

    let mut out = String::new();
    for i in 1..=n {
        out.push_str(&format!("{}\n", ans[i]));
    }
    print!("{out}");
}
