use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // p[1..=n] stores chair identifiers (values assumed within 1..=n as per C code usage)
    let mut p = vec![0usize; n + 1];
    // hi is used first as counts, then reset and reused as per-value fill index / remaining count
    let mut hi = vec![0usize; n + 1];

    for i in 1..=n {
        let a: usize = it.next().unwrap().parse().unwrap();
        p[i] = a;
        if a <= n {
            hi[a] += 1;
        } else {
            // Spec says values are within a defined range; C code would go out of bounds.
            // We keep behavior defined by assuming valid input.
            // If invalid, we ignore here to avoid panic; output will differ from C in that case.
        }
    }

    // q[a] holds positions i where p[i] == a, in input order
    let mut q: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for a in 1..=n {
        q[a] = Vec::with_capacity(hi[a]);
    }

    // Reset hi to 0 (C: memset(hi, 0, (n+1)<<2))
    for v in hi.iter_mut() {
        *v = 0;
    }

    for i in 1..=n {
        let a = p[i];
        if a <= n {
            q[a].push(i);
            hi[a] += 1;
        }
    }

    // sp is a stack of positions
    let mut sp: Vec<usize> = Vec::with_capacity(n);
    let mut ans = vec![0usize; n + 1];

    // Two passes as in C: for (j=0; j<2; j++) for (i=1; i<=n; i++)
    for _j in 0..2 {
        for i in 1..=n {
            // while (hi[i]) sp[top++] = q[i][--hi[i]];
            while hi[i] > 0 {
                hi[i] -= 1;
                let pos = q[i][hi[i]];
                sp.push(pos);
            }
            // if (!ans[i] && top) ans[i] = sp[--top];
            if ans[i] == 0 {
                if let Some(pos) = sp.pop() {
                    ans[i] = pos;
                }
            }
        }
    }

    // Output ans[1..=n], one per line
    let mut out = String::new();
    for i in 1..=n {
        out.push_str(&format!("{}\n", ans[i]));
    }
    print!("{out}");
}
