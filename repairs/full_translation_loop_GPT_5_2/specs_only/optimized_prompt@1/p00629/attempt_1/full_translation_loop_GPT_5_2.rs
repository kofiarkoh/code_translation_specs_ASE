use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Record {
    a: i32,
    b: i32,
    c: i32,
    id: i32,
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // No more datasets.
        }
        let n: i32 = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break, // Spec: invalid formats not handled; narrow behavior: stop.
        };

        // Spec: first integer non-negative; if zero, terminate without processing further.
        if n == 0 {
            break;
        }

        // Spec: maximum 1000 records per dataset (precondition). We do not enforce beyond reading.
        let n_usize = if n > 0 { n as usize } else { 0 };

        let mut records: Vec<Record> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            // Each record has four integer values.
            // If input is incomplete, stop processing further (narrowest reasonable behavior).
            let a = match it.next().and_then(|s| s.parse::<i32>().ok()) {
                Some(v) => v,
                None => return,
            };
            let b = match it.next().and_then(|s| s.parse::<i32>().ok()) {
                Some(v) => v,
                None => return,
            };
            let c = match it.next().and_then(|s| s.parse::<i32>().ok()) {
                Some(v) => v,
                None => return,
            };
            let id = match it.next().and_then(|s| s.parse::<i32>().ok()) {
                Some(v) => v,
                None => return,
            };
            records.push(Record { a, b, c, id });
        }

        // Spec mentions sorting based on "specific criteria of attributes" but does not define them.
        // Narrowest reasonable interpretation: sort lexicographically by the first three attributes,
        // then by identifier to make ordering deterministic.
        records.sort_by(|r1, r2| {
            match r1.a.cmp(&r2.a) {
                Ordering::Equal => match r1.b.cmp(&r2.b) {
                    Ordering::Equal => match r1.c.cmp(&r2.c) {
                        Ordering::Equal => r1.id.cmp(&r2.id),
                        other => other,
                    },
                    other => other,
                },
                other => other,
            }
        });

        // Spec: output up to 25 identifiers, "based on frequency constraints".
        // Frequency constraints are not specified; narrowest reasonable interpretation:
        // do not output the same identifier more than once per dataset.
        let mut out_count = 0usize;
        let mut seen: Vec<i32> = Vec::new(); // small (<=25) in practice; linear search is fine.

        let mut output = String::new();
        for r in records.iter() {
            if out_count >= 25 {
                break;
            }
            if seen.iter().any(|&x| x == r.id) {
                continue;
            }
            seen.push(r.id);
            output.push_str(&format!("{}\n", r.id));
            out_count += 1;
        }

        print!("{output}");
    }
}
