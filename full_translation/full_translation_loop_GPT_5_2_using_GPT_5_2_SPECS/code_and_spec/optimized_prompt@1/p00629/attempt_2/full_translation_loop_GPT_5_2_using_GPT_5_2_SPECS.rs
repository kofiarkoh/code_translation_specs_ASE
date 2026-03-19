use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Record {
    i: i32,    // record identifier
    u: usize,  // group identifier (0..=1000)
    a: i32,    // primary ranking value (score)
    p: i32,    // secondary ranking value (priority)
}

fn main() {
    // Fast input: read all, parse as integers.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut out = String::new();

    loop {
        let n = match it.next() {
            Some(x) => x.parse::<usize>().unwrap(),
            None => break,
        };
        if n == 0 {
            break;
        }

        let mut records = Vec::with_capacity(n);
        for _ in 0..n {
            let i = it.next().unwrap().parse::<i32>().unwrap();
            let u = it.next().unwrap().parse::<usize>().unwrap();
            let a = it.next().unwrap().parse::<i32>().unwrap();
            let p = it.next().unwrap().parse::<i32>().unwrap();
            records.push(Record { i, u, a, p });
        }

        // Spec: order by higher score (a), then lower priority (p), then lower id (i).
        records.sort_unstable_by(|r1, r2| {
            r2.a.cmp(&r1.a)
                .then_with(|| r1.p.cmp(&r2.p))
                .then_with(|| r1.i.cmp(&r2.i))
        });

        // Per spec, quota is based on number of *selected* so far.
        // The C code increments UU on consideration (not selection), which can reduce selections.
        // We follow the SPEC as the source-of-truth to avoid mismatches.
        let mut selected_per_group = [0u8; 1001];
        let mut selected: Vec<i32> = Vec::with_capacity(26);

        for r in records.iter() {
            if selected.len() >= 26 {
                break;
            }

            let quota = if selected.len() < 10 {
                3u8
            } else if selected.len() < 20 {
                2u8
            } else {
                1u8
            };

            let cnt = &mut selected_per_group[r.u];
            if *cnt < quota {
                *cnt += 1;
                selected.push(r.i);
            }
        }

        for id in selected {
            out.push_str(id.to_string().as_str());
            out.push('\n');
        }
    }

    print!("{out}");
}
