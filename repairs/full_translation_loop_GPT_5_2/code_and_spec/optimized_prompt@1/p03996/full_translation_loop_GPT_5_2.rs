use std::io::{self, Read};

#[derive(Clone, Copy, Default)]
struct Node {
    prev: Option<usize>,
    next: Option<usize>,
    id: usize,
    count: i32,
}

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m_in: i32 = it.next().unwrap().parse().unwrap();
    let q: usize = it.next().unwrap().parse().unwrap();

    // Preconditions say operations are in range 0..=m; C code uses 0..=m indexing.
    // If m is negative, C would misbehave; spec says non-negative, so we assume m >= 0.
    let m: usize = m_in as usize;

    // a[1..=q]
    let mut a = vec![0usize; q + 1];
    for i in 1..=q {
        a[i] = it.next().unwrap().parse::<usize>().unwrap();
    }

    // Prepare nodes d[0..=m]
    let mut d = vec![Node::default(); m + 1];
    for i in 0..=m {
        d[i].id = i;
    }

    // Build linked structure (mirrors C logic)
    let mut p: usize = 0; // index of current tail; initially d[0]
    d[0].count = n;

    for i in (1..=q).rev() {
        let idx = a[i];

        if d[idx].count == 0 {
            d[idx].prev = Some(p);
            d[p].next = Some(idx);
            p = idx;
        }

        // In C: d[a[i]].count = min(d[a[i]].count+1, d[a[i]].prev->count);
        // Note: this assumes prev is non-NULL; the algorithm relies on having linked it
        // before first update when count==0. We preserve that behavior.
        let prev_idx = d[idx].prev.expect("prev must exist when updating count");
        let candidate = d[idx].count + 1;
        let limit = d[prev_idx].count;
        d[idx].count = if candidate < limit { candidate } else { limit };
    }

    // Core check
    let mut complete = vec![false; m + 1];
    let mut allow: usize = 0;

    // Traverse from head d[0] following next pointers
    let mut cur: Option<usize> = Some(0);

    while let Some(ci) = cur {
        if d[ci].count != n {
            break;
        }
        complete[d[ci].id] = true;
        while allow <= m && complete[allow] {
            allow += 1;
        }
        cur = d[ci].next;
    }

    while let Some(ci) = cur {
        if d[ci].id != allow {
            print!("No\n");
            return;
        }
        complete[allow] = true;
        while allow <= m && complete[allow] {
            allow += 1;
        }
        cur = d[ci].next;
    }

    print!("Yes\n");
}
