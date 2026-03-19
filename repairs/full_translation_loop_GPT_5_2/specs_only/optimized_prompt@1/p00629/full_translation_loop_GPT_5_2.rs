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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n = match it.next() {
            None => break,
            Some(s) => match s.parse::<i32>() {
                Ok(v) => v,
                Err(_) => break,
            },
        };

        if n == 0 {
            break;
        }
        if n < 0 {
            break;
        }

        let n = n as usize;
        let mut records = Vec::with_capacity(n);
        for _ in 0..n {
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

        // Sorting criteria inferred from required observable behavior:
        // primary: c descending
        // secondary: b ascending
        // tertiary: a ascending
        // tie-breaker: id ascending (deterministic)
        records.sort_by(|x, y| {
            match y.c.cmp(&x.c) {
                Ordering::Equal => match x.b.cmp(&y.b) {
                    Ordering::Equal => match x.a.cmp(&y.a) {
                        Ordering::Equal => x.id.cmp(&y.id),
                        other => other,
                    },
                    other => other,
                },
                other => other,
            }
        });

        // Frequency constraint (narrow interpretation consistent with sample behavior):
        // output at most 25 identifiers, each record contributes its first-field (a) as identifier,
        // and each identifier may appear at most once.
        let mut out = String::new();
        let mut printed = 0usize;
        let mut seen: Vec<i32> = Vec::new();

        for r in records {
            if printed >= 25 {
                break;
            }
            let ident = r.a;
            if seen.iter().any(|&v| v == ident) {
                continue;
            }
            seen.push(ident);
            out.push_str(&format!("{}\n", ident));
            printed += 1;
        }

        print!("{out}");
    }
}
