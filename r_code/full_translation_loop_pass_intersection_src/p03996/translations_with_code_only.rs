use std::io::{self, Read};

#[derive(Clone, Copy, Default)]
struct Data {
    prev: Option<usize>, // index into d
    next: Option<usize>, // index into d
    id: i32,
    count: i32,
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;
    let q: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;

    // a[1..=q]
    let mut a = vec![0usize; q + 1];
    for i in 1..=q {
        a[i] = it.next().unwrap().parse::<i32>().unwrap() as usize;
    }

    // Prepare d[0..=m]
    let mut d = vec![Data::default(); m + 1];
    for i in 0..=m {
        d[i].id = i as i32;
    }

    // p is an index into d; start at d[0]
    let mut p: usize = 0;
    d[0].count = n;

    // Build linked list via indices, emulating the C pointer logic.
    // Note: The C code reads d[a[i]].prev->count even when prev is NULL for the first inserted node,
    // which is undefined behavior. We emulate the *intended* behavior by treating "no prev" as d[0].
    for i in (1..=q).rev() {
        let idx = a[i];
        if d[idx].count == 0 {
            d[idx].prev = Some(p);
            d[p].next = Some(idx);
            p = idx;
        }
        let prev_idx = d[idx].prev.unwrap_or(0);
        let prev_count = d[prev_idx].count;
        let new_count = d[idx].count + 1;
        d[idx].count = if new_count < prev_count { new_count } else { prev_count };
    }

    // Core
    let mut complete = vec![false; m + 1];
    let mut allow: usize = 0;

    // d pointer starts at d[0]
    let mut cur: Option<usize> = Some(0);

    while let Some(ci) = cur {
        if d[ci].count != n {
            break;
        }
        complete[d[ci].id as usize] = true;
        while allow <= m && complete[allow] {
            allow += 1;
        }
        cur = d[ci].next;
    }

    while let Some(ci) = cur {
        if d[ci].id as usize != allow {
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
