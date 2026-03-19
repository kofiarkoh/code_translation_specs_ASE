use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Op {
    kind: u8, // b'M' for mark; anything else is query by behavior
    x: usize,
}

fn find_active(mut v: usize, parent: &[usize], active: &[i32]) -> usize {
    // Follow parent links until reaching a node with positive active count.
    // Preconditions guarantee termination (eventually reaches an active value).
    while active[v] <= 0 {
        v = parent[v];
    }
    v
}

fn main() {
    // Read entire stdin, parse as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let q: usize = it.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        // parent[1] is unused/self; parents for 2..=N are provided.
        let mut parent = vec![0usize; n + 1];
        parent[1] = 1;
        for k in 2..=n {
            parent[k] = it.next().unwrap().parse().unwrap();
        }

        let mut ops = Vec::with_capacity(q);
        for _ in 0..q {
            let c = it.next().unwrap().as_bytes()[0];
            let x: usize = it.next().unwrap().parse().unwrap();
            ops.push(Op { kind: c, x });
        }

        // Active counts: initial active multiset contains value 1 once.
        let mut active = vec![0i32; n + 1];
        active[1] = 1;

        let mut sum: i64 = 0;

        for op in ops {
            if op.kind == b'M' {
                // Mark operation: increase active count (possibly multiple times).
                // i32 is sufficient for counts under given constraints.
                active[op.x] = active[op.x].wrapping_add(1);
            } else {
                // Query operation: any non-'M' character is treated as query.
                let res = find_active(op.x, &parent, &active);
                sum = sum.wrapping_add(res as i64);
            }
        }

        out.push_str(&format!("{sum}\n"));
    }

    print!("{out}");
}
