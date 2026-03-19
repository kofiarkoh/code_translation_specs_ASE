use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 tokens (matches C's %d behavior for typical inputs).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let x: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => std::process::exit(1),
    };
    let y: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => std::process::exit(1),
    };
    let a: usize = match it.next() {
        Some(s) => s.parse::<i32>().unwrap() as usize,
        None => std::process::exit(1),
    };
    let b: usize = match it.next() {
        Some(s) => s.parse::<i32>().unwrap() as usize,
        None => std::process::exit(1),
    };
    let c: usize = match it.next() {
        Some(s) => s.parse::<i32>().unwrap() as usize,
        None => std::process::exit(1),
    };

    let mut p: Vec<i32> = Vec::with_capacity(a);
    let mut q: Vec<i32> = Vec::with_capacity(b);
    let mut r: Vec<i32> = Vec::with_capacity(c);

    for _ in 0..a {
        let v: i32 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => std::process::exit(1),
        };
        p.push(v);
    }
    for _ in 0..b {
        let v: i32 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => std::process::exit(1),
        };
        q.push(v);
    }
    for _ in 0..c {
        let v: i32 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => std::process::exit(1),
        };
        r.push(v);
    }

    // C comparator sorts descending.
    p.sort_by(|u, v| v.cmp(u));
    q.sort_by(|u, v| v.cmp(u));
    r.sort_by(|u, v| v.cmp(u));

    // Indices are signed in C; preserve the same control flow.
    let mut asumi: i32 = x - 1;
    let mut iguti: i32 = y - 1;

    for i in 0..c {
        if asumi < 0 && iguti < 0 {
            break;
        } else if asumi < 0
            || (iguti >= 0 && p[asumi as usize] > q[iguti as usize])
        {
            // Replace in q
            let qi = iguti as usize;
            if r[i] > q[qi] {
                q[qi] = r[i];
                iguti -= 1;
            } else {
                break;
            }
        } else {
            // Replace in p
            let pi = asumi as usize;
            if r[i] > p[pi] {
                p[pi] = r[i];
                asumi -= 1;
            } else {
                break;
            }
        }
    }

    let mut kitaeri: i64 = 0;
    for i in 0..(x as usize) {
        kitaeri += p[i] as i64;
    }
    for i in 0..(y as usize) {
        kitaeri += q[i] as i64;
    }

    print!("{}\n", kitaeri);
}
