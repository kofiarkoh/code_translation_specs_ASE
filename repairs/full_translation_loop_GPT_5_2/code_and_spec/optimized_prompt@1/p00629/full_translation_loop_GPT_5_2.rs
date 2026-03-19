use std::io::{self, Read};

#[derive(Clone, Copy)]
struct T {
    i: i32,
    u: usize,
    a: i32,
    p: i32,
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // No more datasets.
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break; // Termination condition per spec/C.
        }

        // Spec says up to 1000 records; C code uses TT[300] but does not guard.
        // We follow the spec and allow up to 1000.
        let mut tt: Vec<T> = Vec::with_capacity(n);
        for _ in 0..n {
            let i_val: i32 = it.next().unwrap().parse().unwrap();
            let u_val: usize = it.next().unwrap().parse().unwrap();
            let a_val: i32 = it.next().unwrap().parse().unwrap();
            let p_val: i32 = it.next().unwrap().parse().unwrap();
            tt.push(T {
                i: i_val,
                u: u_val,
                a: a_val,
                p: p_val,
            });
        }

        // Sort criteria (matching the bubble sort comparisons):
        // - A descending
        // - P ascending
        // - I ascending
        tt.sort_by(|x, y| {
            // Note: reverse for A (descending)
            y.a.cmp(&x.a)
                .then_with(|| x.p.cmp(&y.p))
                .then_with(|| x.i.cmp(&y.i))
        });

        // Frequency constraints by U:
        // first 10 picks: allow up to 3 per U
        // next 10 picks (up to 20): allow up to 2 per U
        // next 6 picks (up to 26): allow up to 1 per U
        let mut uu = vec![0u32; 1001];
        let mut ss: Vec<i32> = Vec::with_capacity(26);

        for rec in tt.iter() {
            if rec.u < uu.len() {
                uu[rec.u] += 1;
            } else {
                // C would index out of bounds (undefined behavior). Spec does not define this.
                // We ignore such records to avoid panics while keeping behavior defined.
                continue;
            }

            let head = ss.len();
            let count = uu[rec.u];

            if head < 10 && count <= 3 {
                ss.push(rec.i);
            } else if head < 20 && count <= 2 {
                ss.push(rec.i);
            } else if head < 26 && count <= 1 {
                ss.push(rec.i);
            }

            if ss.len() >= 26 {
                // No further outputs possible.
                // (C continues scanning but cannot add more; we can stop early.)
                break;
            }
        }

        for id in ss {
            out.push_str(&format!("{id}\n"));
        }
    }

    print!("{out}");
}
