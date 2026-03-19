use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Rect {
    id: usize,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    used: bool,
}

fn calc(t: &mut [Rect], ans: &mut [i32], order: &mut i32) {
    // Sort by (x1, y1) ascending, matching C's qsort+cmp.
    t.sort_by(|a, b| match a.x1.cmp(&b.x1) {
        std::cmp::Ordering::Equal => a.y1.cmp(&b.y1),
        other => other,
    });

    // Assign first rectangle.
    *order += 1;
    ans[t[0].id] = *order;
    t[0].used = true;

    // Determine y and x as in C.
    let y = t[0].y2;
    let mut x: i32 = 501;
    for r in t.iter() {
        if r.used {
            continue;
        }
        if r.y1 < y && y < r.y2 {
            x = r.x1;
            break;
        }
    }

    // Collect subset into tmp and mark them used in t.
    let mut tmp: Vec<Rect> = Vec::with_capacity(10);
    for r in t.iter_mut() {
        if r.used {
            continue;
        }
        if r.y2 <= y && r.x2 <= x {
            tmp.push(*r);
            r.used = true;
        }
    }
    if !tmp.is_empty() {
        calc(&mut tmp[..], ans, order);
    }

    // Important: In the C code, the loop calls calc(t+i, n-i) for every i where t[i] is unused.
    // But after the first such call, that call will assign all remaining rectangles in that suffix,
    // making later indices used. In Rust, naively iterating i=0..n and recursing can re-enter
    // already-processed suffixes due to re-sorting, causing excessive recursion / timeout.
    //
    // We therefore mimic the effective behavior: find the first unused index and recurse once.
    if let Some(i) = t.iter().position(|r| !r.used) {
        calc(&mut t[i..], ans, order);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut first_case = true;
    let mut out = String::new();

    while let Some(tok) = it.next() {
        let n: usize = tok.parse().unwrap();
        if n == 0 {
            break;
        }

        let mut tbl: Vec<Rect> = Vec::with_capacity(n);
        for i in 0..n {
            let x1: i32 = it.next().unwrap().parse().unwrap();
            let y1: i32 = it.next().unwrap().parse().unwrap();
            let x2: i32 = it.next().unwrap().parse().unwrap();
            let y2: i32 = it.next().unwrap().parse().unwrap();

            let (minx, maxx) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            let (miny, maxy) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

            tbl.push(Rect {
                id: i,
                x1: minx,
                y1: miny,
                x2: maxx,
                y2: maxy,
                used: false,
            });
        }

        let mut ans = vec![0i32; n];
        let mut order: i32 = 0;
        calc(&mut tbl[..], &mut ans[..], &mut order);

        if !first_case {
            out.push('\n');
        }
        first_case = false;

        for v in ans {
            out.push_str(&format!("{v}\n"));
        }
    }

    print!("{out}");
}
