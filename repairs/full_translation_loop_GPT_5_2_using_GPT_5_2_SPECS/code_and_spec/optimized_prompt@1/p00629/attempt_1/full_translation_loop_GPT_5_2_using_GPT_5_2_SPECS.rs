use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Record {
    i: i32, // record identifier
    u: usize, // group identifier (0..=1000)
    a: i32, // primary ranking value (score)
    p: i32, // secondary ranking value (priority)
}

fn main() {
    // No explicit error handling per spec: assume well-formed input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // input ended unexpectedly; unspecified, but we stop.
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break; // terminating dataset produces no output
        }

        let mut records: Vec<Record> = Vec::with_capacity(n);
        for _ in 0..n {
            let i: i32 = it.next().unwrap().parse().unwrap();
            let u: usize = it.next().unwrap().parse().unwrap();
            let a: i32 = it.next().unwrap().parse().unwrap();
            let p: i32 = it.next().unwrap().parse().unwrap();
            records.push(Record { i, u, a, p });
        }

        // Order by: higher A first, then lower P, then lower I.
        records.sort_by(|r1, r2| {
            r2.a.cmp(&r1.a)
                .then_with(|| r1.p.cmp(&r2.p))
                .then_with(|| r1.i.cmp(&r2.i))
        });

        let mut per_group = [0u32; 1001];
        let mut selected: Vec<i32> = Vec::with_capacity(26);

        for r in records {
            if selected.len() >= 26 {
                break;
            }

            // C code increments the group counter on consideration (even if not selected).
            // This matches the given C behavior; the spec's "per-group selection count"
            // wording is slightly ambiguous, but we follow the source-of-truth behavior.
            per_group[r.u] += 1;
            let cnt = per_group[r.u];

            let head = selected.len();
            if head < 10 {
                if cnt <= 3 {
                    selected.push(r.i);
                }
            } else if head < 20 {
                if cnt <= 2 {
                    selected.push(r.i);
                }
            } else {
                // head is 20..=25 here due to early break at 26
                if cnt <= 1 {
                    selected.push(r.i);
                }
            }
        }

        for id in selected {
            out.push_str(&format!("{id}\n"));
        }
    }

    print!("{out}");
}
