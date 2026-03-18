use std::cmp::Ordering;
use std::io::{self, Read};

const MAX_N: usize = 1_000_000;
const NIL: i32 = -1;

#[derive(Clone, Copy, Default)]
struct Point {
    id: i32,
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Default)]
struct Node {
    location: i32, // index into p[]
    l: i32,
    r: i32,
}

fn make_kd_tree(p: &mut [Point], t: &mut [Node], l: i32, r: i32, depth: i32, np: &mut i32) -> i32 {
    if l >= r {
        return NIL;
    }

    let len = (r - l) as usize;
    if len >= 2 {
        let lo = l as usize;
        let hi = r as usize;
        if depth % 2 == 0 {
            // C qsort is not stable; Rust's sort_unstable matches that property.
            p[lo..hi].sort_unstable_by(|a, b| a.x.cmp(&b.x));
        } else {
            p[lo..hi].sort_unstable_by(|a, b| a.y.cmp(&b.y));
        }
    }

    let mid = (l + r) / 2;
    let s = *np;
    *np += 1;

    let si = s as usize;
    t[si].location = mid;
    t[si].l = make_kd_tree(p, t, l, mid, depth + 1, np);
    t[si].r = make_kd_tree(p, t, mid + 1, r, depth + 1, np);
    s
}

fn find(
    p: &[Point],
    t: &[Node],
    pid: &mut Vec<i32>,
    v: i32,
    sx: i32,
    tx: i32,
    sy: i32,
    ty: i32,
    depth: i32,
) {
    let vi = v as usize;
    let loc = t[vi].location as usize;
    let x = p[loc].x;
    let y = p[loc].y;

    if sx <= x && x <= tx && sy <= y && y <= ty {
        pid.push(p[loc].id);
    }

    if depth % 2 == 0 {
        let left = t[vi].l;
        if left != NIL && sx <= x {
            find(p, t, pid, left, sx, tx, sy, ty, depth + 1);
        }
        let right = t[vi].r;
        if right != NIL && x <= tx {
            find(p, t, pid, right, sx, tx, sy, ty, depth + 1);
        }
    } else {
        let left = t[vi].l;
        if left != NIL && sy <= y {
            find(p, t, pid, left, sx, tx, sy, ty, depth + 1);
        }
        let right = t[vi].r;
        if right != NIL && y <= ty {
            find(p, t, pid, right, sx, tx, sy, ty, depth + 1);
        }
    }
}

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // Mirror the C global arrays with fixed maximum capacity.
    // (C would overflow if n > MAX_N; we keep the same assumption.)
    assert!(n <= MAX_N);

    let mut p = vec![Point::default(); MAX_N];
    let mut t = vec![Node::default(); MAX_N];

    for i in 0..n {
        let x: i32 = it.next().unwrap().parse().unwrap();
        let y: i32 = it.next().unwrap().parse().unwrap();
        p[i].id = i as i32;
        p[i].x = x;
        p[i].y = y;
        t[i].l = NIL;
        t[i].r = NIL;
    }

    let mut np: i32 = 0;
    let root = make_kd_tree(&mut p[..n], &mut t, 0, n as i32, 0, &mut np);

    let q: usize = it.next().unwrap().parse().unwrap();
    let mut out = String::new();

    for _ in 0..q {
        let sx: i32 = it.next().unwrap().parse().unwrap();
        let tx: i32 = it.next().unwrap().parse().unwrap();
        let sy: i32 = it.next().unwrap().parse().unwrap();
        let ty: i32 = it.next().unwrap().parse().unwrap();

        let mut pid: Vec<i32> = Vec::new();
        if root != NIL {
            find(&p[..n], &t, &mut pid, root, sx, tx, sy, ty, 0);
        }

        // C qsort on int array; use unstable sort to match non-stability.
        pid.sort_unstable_by(|a, b| a.cmp(b));

        for id in pid {
            out.push_str(&format!("{id}\n"));
        }
        out.push('\n');
    }

    print!("{out}");
}
