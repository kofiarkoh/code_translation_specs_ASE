use std::io::{self, Read};

#[derive(Clone)]
struct Record {
    label: char,
    key: i64,
    pos: usize, // original input position (for stability check only)
}

fn main() {
    // Read entire stdin; no explicit error handling per specification.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Specification note: if n == 0, behavior is not inferable from code.
    // We choose the narrowest reasonable behavior: do nothing (no output).
    if n == 0 {
        return;
    }

    let mut records: Vec<Record> = Vec::with_capacity(n);
    for pos in 0..n {
        let label_token = it.next().unwrap();
        let label = label_token.chars().next().unwrap();
        let key: i64 = it.next().unwrap().parse().unwrap();
        records.push(Record { label, key, pos });
    }

    // Sort by key in nondecreasing order. We intentionally do not use a stable sort,
    // because the task is to determine whether the produced ordering is stable.
    records.sort_unstable_by(|a, b| a.key.cmp(&b.key));

    // Determine stability of the produced output order w.r.t. original positions.
    // Stable iff within each equal-key group, original positions are nondecreasing.
    let mut stable = true;
    for i in 1..records.len() {
        if records[i - 1].key == records[i].key && records[i - 1].pos > records[i].pos {
            stable = false;
            break;
        }
    }

    let mut out = String::new();
    if stable {
        out.push_str("Stable\n");
    } else {
        out.push_str("Not stable\n");
    }
    for r in records {
        out.push(r.label);
        out.push(' ');
        out.push_str(&r.key.to_string());
        out.push('\n');
    }

    print!("{out}");
}
