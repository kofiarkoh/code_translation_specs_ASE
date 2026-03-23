use std::io::{self, Read};

#[derive(Clone)]
struct Record {
    label: char,
    key: i32,
    pos: usize, // original input position
}

fn partition(a: &mut [Record], p: isize, r: isize) -> isize {
    let x = a[r as usize].key; // pivot key is last element
    let mut i = p - 1;
    for j in p..r {
        if a[j as usize].key <= x {
            i += 1;
            a.swap(i as usize, j as usize);
        }
    }
    a.swap((i + 1) as usize, r as usize);
    i + 1
}

fn quicksort(a: &mut [Record], p: isize, r: isize) {
    if p < r {
        let q = partition(a, p, r);
        quicksort(a, p, q - 1);
        quicksort(a, q + 1, r);
    }
}

fn main() {
    // Read entire stdin; specification assumes formatted scanning with whitespace before label.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let mut a: Vec<Record> = Vec::with_capacity(n);
    for pos in 0..n {
        let label_tok = it.next().unwrap();
        let label = label_tok.chars().next().unwrap(); // single-character label
        let key: i32 = it.next().unwrap().parse().unwrap();
        a.push(Record { label, key, pos });
    }

    if n > 0 {
        quicksort(&mut a, 0, (n as isize) - 1);
    } else {
        // Behavior unspecified for n <= 0; keep as-is without inventing extra behavior.
    }

    let mut stable = true;
    for i in 0..n {
        for j in (i + 1)..n {
            if a[i].key == a[j].key && a[i].pos > a[j].pos {
                stable = false;
                break;
            }
        }
        if !stable {
            break;
        }
    }

    let mut out = String::new();
    if stable {
        out.push_str("Stable\n");
    } else {
        out.push_str("Not stable\n");
    }
    for rec in &a {
        out.push(rec.label);
        out.push(' ');
        out.push_str(&rec.key.to_string());
        out.push('\n');
    }

    print!("{out}");
}
