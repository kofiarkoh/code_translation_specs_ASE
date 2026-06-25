use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Record {
    id: i32,
    group: usize,
    primary: i32,
    secondary: i32,
}

fn tier_limit(selected_so_far: usize) -> usize {
    // selected_so_far is the number already selected before considering the next selection.
    // Next selection position is selected_so_far + 1.
    let pos = selected_so_far + 1;
    if pos <= 10 {
        3
    } else if pos <= 20 {
        2
    } else {
        1
    }
}

fn main() {
    // No explicit error handling per specification; if input is malformed, behavior is unspecified.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // unspecified; treat as end of input
        }
        let n: usize = n_opt.unwrap().parse().unwrap();

        if n == 0 {
            break; // terminating dataset
        }

        let mut records: Vec<Record> = Vec::with_capacity(n);
        for _ in 0..n {
            let id: i32 = it.next().unwrap().parse().unwrap();
            let group_i32: i32 = it.next().unwrap().parse().unwrap();
            let primary: i32 = it.next().unwrap().parse().unwrap();
            let secondary: i32 = it.next().unwrap().parse().unwrap();

            // Preconditions state group identifiers are non-negative and within range.
            // If violated, behavior is unspecified; we still parse and cast.
            let group: usize = group_i32 as usize;

            records.push(Record {
                id,
                group,
                primary,
                secondary,
            });
        }

        // Rank:
        // 1) higher primary first
        // 2) lower secondary first
        // 3) lower id first
        records.sort_by(|a, b| {
            match b.primary.cmp(&a.primary) {
                Ordering::Equal => match a.secondary.cmp(&b.secondary) {
                    Ordering::Equal => a.id.cmp(&b.id),
                    other => other,
                },
                other => other,
            }
        });

        // Fixed internal counting range assumed 0..=1000 inclusive.
        // No explicit error handling; if group is out of range, behavior is unspecified.
        let mut group_counts = vec![0usize; 1001];

        let mut selected = 0usize;
        for r in records.iter() {
            if selected >= 26 {
                break;
            }

            let limit = tier_limit(selected);

            if r.group < group_counts.len() && group_counts[r.group] < limit {
                group_counts[r.group] += 1;
                selected += 1;
                out.push_str(&format!("{}\n", r.id));
            }
        }
    }

    print!("{out}");
}
