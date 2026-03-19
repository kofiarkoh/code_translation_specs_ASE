use std::cmp::Ordering;
use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let q: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // Maintain an in-memory ordered set of integers with uniqueness.
    // Use BTreeSet to preserve ascending order and allow range iteration.
    let mut set = std::collections::BTreeSet::<i64>::new();

    let mut out = String::new();

    for _ in 0..q {
        let cmd: i32 = it.next().unwrap().parse().unwrap();
        match cmd {
            0 => {
                let x: i64 = it.next().unwrap().parse().unwrap();
                set.insert(x);
                out.push_str(&format!("{}\n", set.len()));
            }
            1 => {
                let x: i64 = it.next().unwrap().parse().unwrap();
                out.push_str(if set.contains(&x) { "1\n" } else { "0\n" });
            }
            2 => {
                let x: i64 = it.next().unwrap().parse().unwrap();
                set.remove(&x);
            }
            3 => {
                let lo: i64 = it.next().unwrap().parse().unwrap();
                let hi: i64 = it.next().unwrap().parse().unwrap();

                // Inclusive range dump in ascending order.
                // Preconditions in the spec guarantee the dump is only issued when:
                // - the set is non-empty
                // - there exists at least one element >= lo
                // Behavior is otherwise undefined; we do not add extra checks.
                for &v in set.range(lo..=hi) {
                    out.push_str(&format!("{}\n", v));
                }
            }
            _ => {
                // Spec: invalid command codes have undefined behavior.
                // Narrowest reasonable interpretation: do nothing (no defined handling).
            }
        }
    }

    print!("{out}");
}
