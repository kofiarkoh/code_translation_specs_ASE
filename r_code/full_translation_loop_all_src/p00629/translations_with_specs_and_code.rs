use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Record {
    i: i32, // record identifier
    u: usize, // group identifier (0..=1000 per spec)
    a: i32, // primary score (higher is better)
    p: i32, // secondary score (lower is better)
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
            break; // unspecified by spec; just stop if input ends
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break; // terminating dataset
        }

        // Read records
        let mut records: Vec<Record> = Vec::with_capacity(n);
        for _ in 0..n {
            let i: i32 = it.next().unwrap().parse().unwrap();
            let u_i32: i32 = it.next().unwrap().parse().unwrap();
            let a: i32 = it.next().unwrap().parse().unwrap();
            let p: i32 = it.next().unwrap().parse().unwrap();
            records.push(Record {
                i,
                u: u_i32 as usize,
                a,
                p,
            });
        }

        // Rank by: higher A, then lower P, then lower I.
        records.sort_by(|r1, r2| {
            r2.a.cmp(&r1.a) // descending A
                .then_with(|| r1.p.cmp(&r2.p)) // ascending P
                .then_with(|| r1.i.cmp(&r2.i)) // ascending I
        });

        // Selection with tiered per-group quotas.
        // Note: The C code increments the per-group counter for every considered record,
        // not only for selected ones. This matches the spec's "at the time it is considered"
        // and is required for identical behavior.
        let mut uu = vec![0i32; 1001]; // per-group considered count
        let mut selected: Vec<i32> = Vec::with_capacity(26);

        for r in records.iter() {
            if r.u < uu.len() {
                uu[r.u] += 1;
            } else {
                // Preconditions say this won't happen; behavior unspecified otherwise.
                // We mimic "no explicit error handling" by allowing a panic if violated.
                panic!("group id out of range");
            }

            let head = selected.len();
            if head < 10 {
                if uu[r.u] <= 3 {
                    selected.push(r.i);
                }
            } else if head < 20 {
                if uu[r.u] <= 2 {
                    selected.push(r.i);
                }
            } else if head < 26 {
                if uu[r.u] <= 1 {
                    selected.push(r.i);
                }
            } else {
                break;
            }
        }

        for id in selected {
            out.push_str(&format!("{id}\n"));
        }
    }

    print!("{out}");
}
