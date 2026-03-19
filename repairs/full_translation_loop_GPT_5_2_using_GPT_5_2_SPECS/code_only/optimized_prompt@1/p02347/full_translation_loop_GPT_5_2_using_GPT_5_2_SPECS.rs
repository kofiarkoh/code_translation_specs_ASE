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
    l: i32,        // index into t[] or NIL
    r: i32,        // index into t[] or NIL
}

fn make_kd_tree(p: &mut [Point], t: &mut [Node], np: &mut i32, l: i32, r: i32, depth: i32) -> i32 {
    if l >= r {
        return NIL;
    }

    // In C: if (r - l >= 2) qsort(&p[l], r-l, ...)
    // Rust: sort the sub-slice p[l..r] by x or y. This is not stable, matching qsort's lack of stability.
    if r - l >= 2 {
        let (lu, ru) = (l as usize, r as usize);
        if depth % 2 == 0 {
            p[lu..ru].sort_unstable_by(|a, b| a.x.cmp(&b.x));
        } else {
            p[lu..ru].sort_unstable_by(|a, b| a.y.cmp(&b.y));
        }
    }

    // C integer division truncates toward zero; l,r are non-negative here, so this matches.
    let mid = (l + r) / 2;

    let s = *np;
    *np += 1;

    t[s as usize].location = mid;
    t[s as usize].l = make_kd_tree(p, t, np, l, mid, depth + 1);
    t[s as usize].r = make_kd_tree(p, t, np, mid + 1, r, depth + 1);
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
    let loc = t[v as usize].location as usize;
    let x = p[loc].x;
    let y = p[loc].y;

    if sx <= x && x <= tx && sy <= y && y <= ty {
        pid.push(p[loc].id);
    }

    if depth % 2 == 0 {
        let left = t[v as usize].l;
        if left != NIL && sx <= x {
            find(p, t, pid, left, sx, tx, sy, ty, depth + 1);
        }
        let right = t[v as usize].r;
        if right != NIL && x <= tx {
            find(p, t, pid, right, sx, tx, sy, ty, depth + 1);
        }
    } else {
        let left = t[v as usize].l;
        if left != NIL && sy <= y {
            find(p, t, pid, left, sx, tx, sy, ty, depth + 1);
        }
        let right = t[v as usize].r;
        if right != NIL && y <= ty {
            find(p, t, pid, right, sx, tx, sy, ty, depth + 1);
        }
    }
}

fn main() {
    // Read all input, parse as i32 like scanf("%d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse::<i32>().unwrap() as usize,
        None => return,
    };

    // Mirror the C globals with fixed-capacity arrays.
    // Note: C code assumes n <= MAX_N; we do the same.
    let mut p = vec![Point::default(); MAX_N];
    let mut t = vec![Node::default(); MAX_N];

    for i in 0..n {
        let x = it.next().unwrap().parse::<i32>().unwrap();
        let y = it.next().unwrap().parse::<i32>().unwrap();
        p[i] = Point {
            id: i as i32,
            x,
            y,
        };
        t[i].l = NIL;
        t[i].r = NIL;
    }

    let mut np: i32 = 0;
    let root = make_kd_tree(&mut p[..n], &mut t, &mut np, 0, n as i32, 0);

    let q: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;

    let mut out = String::new();
    let mut pid: Vec<i32> = Vec::new();

    for _ in 0..q {
        let sx = it.next().unwrap().parse::<i32>().unwrap();
        let tx = it.next().unwrap().parse::<i32>().unwrap();
        let sy = it.next().unwrap().parse::<i32>().unwrap();
        let ty = it.next().unwrap().parse::<i32>().unwrap();

        pid.clear();
        if root != NIL {
            find(&p[..n], &t, &mut pid, root, sx, tx, sy, ty, 0);
        }

        // C uses qsort with integer comparator; sort_unstable matches.
        pid.sort_unstable_by(|a, b| a.cmp(b));

        for id in &pid {
            out.push_str(&format!("{id}\n"));
        }
        out.push('\n');
    }

    print!("{out}");
}
