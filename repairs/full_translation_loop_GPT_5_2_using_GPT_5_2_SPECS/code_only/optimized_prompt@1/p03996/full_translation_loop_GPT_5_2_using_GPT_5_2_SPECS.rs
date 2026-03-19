use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Node {
    prev: Option<usize>,
    next: Option<usize>,
    id: usize,
    count: i32,
}

fn main() {
    // Read entire stdin, parse as ints like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();
    let q: usize = it.next().unwrap().parse().unwrap();

    // a[1..=q]
    let mut a = vec![0usize; q + 1];
    for i in 1..=q {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    // Prepare nodes d[0..=m]
    let mut d = vec![
        Node {
            prev: None,
            next: None,
            id: 0,
            count: 0
        };
        m + 1
    ];
    for i in 0..=m {
        d[i].id = i;
    }

    // p = d (points to d[0])
    let mut p: usize = 0;
    d[0].count = n;

    // Build linked list via indices.
    // Note: The original C has UB when accessing d[a[i]].prev->count before prev is set.
    // We emulate the *intended* behavior by treating "no prev" as pointing to d[0],
    // which is consistent with the list being anchored at d[0] and p starting at d[0].
    for i in (1..=q).rev() {
        let idx = a[i];
        if d[idx].count == 0 {
            d[idx].prev = Some(p);
            d[p].next = Some(idx);
            p = idx;
        }
        let prev_idx = d[idx].prev.unwrap_or(0);
        let prev_count = d[prev_idx].count;
        let new_count = d[idx].count.saturating_add(1);
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
