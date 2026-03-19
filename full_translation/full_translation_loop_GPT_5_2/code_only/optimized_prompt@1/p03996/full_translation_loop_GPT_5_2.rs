use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Data {
    prev: Option<usize>,
    next: Option<usize>,
    id: i32,
    count: i32,
}

fn main() {
    // Read entire stdin, parse as ints like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;
    let q: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;

    // a[1..=q], a[0] unused; calloc => zero-initialized
    let mut a = vec![0i32; q + 1];
    for i in 1..=q {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    // d[0..=m], calloc => zero-initialized pointers/counts
    let mut d = vec![
        Data {
            prev: None,
            next: None,
            id: 0,
            count: 0
        };
        m + 1
    ];
    for i in 0..=m {
        d[i].id = i as i32;
    }

    // p = d (points to d[0])
    let mut p: usize = 0;
    d[0].count = n;

    // Build linked list via indices, emulating the C logic.
    // Note: The C code can dereference d[a[i]].prev even when it was never set
    // (undefined behavior). We emulate the "intended" behavior by treating a
    // missing prev as 0 (the head), which matches how nodes are first linked.
    for i in (1..=q).rev() {
        let ai = a[i] as isize;
        if ai < 0 || ai as usize > m {
            // Out-of-bounds in C would be UB; we cannot reproduce safely.
            // Keep behavior deterministic: treat as immediate "No" by exiting.
            // (This path should not occur for valid inputs.)
            print!("No\n");
            return;
        }
        let idx = ai as usize;

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

    // Core check
    let mut complete = vec![false; m + 1];
    let mut allow: usize = 0;

    // d pointer starts at head (index 0)
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
