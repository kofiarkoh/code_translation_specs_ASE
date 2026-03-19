use std::cmp::Ordering;
use std::io::{self, Read};

const NIL: i32 = -1;

#[derive(Clone, Copy)]
struct Point {
    id: i32,
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Node {
    location: usize, // index into points array
    l: i32,          // index into nodes array, or NIL
    r: i32,          // index into nodes array, or NIL
}

fn make_kd_tree(points: &mut [Point], nodes: &mut Vec<Node>, l: usize, r: usize, depth: usize) -> i32 {
    if l >= r {
        return NIL;
    }

    // Match C behavior: only sort when there are at least 2 elements.
    if r - l >= 2 {
        if depth % 2 == 0 {
            points[l..r].sort_unstable_by(|a, b| a.x.cmp(&b.x));
        } else {
            points[l..r].sort_unstable_by(|a, b| a.y.cmp(&b.y));
        }
    }

    let mid = (l + r) / 2;
    let s = nodes.len() as i32;
    nodes.push(Node {
        location: mid,
        l: NIL,
        r: NIL,
    });

    let left = make_kd_tree(points, nodes, l, mid, depth + 1);
    let right = make_kd_tree(points, nodes, mid + 1, r, depth + 1);

    // Fill children after recursion (safe because we only append to nodes).
    let idx = s as usize;
    nodes[idx].l = left;
    nodes[idx].r = right;

    s
}

fn find(
    v: i32,
    points: &[Point],
    nodes: &[Node],
    sx: i32,
    tx: i32,
    sy: i32,
    ty: i32,
    depth: usize,
    out_ids: &mut Vec<i32>,
) {
    if v == NIL {
        return;
    }
    let node = nodes[v as usize];
    let pt = points[node.location];
    let x = pt.x;
    let y = pt.y;

    if sx <= x && x <= tx && sy <= y && y <= ty {
        out_ids.push(pt.id);
    }

    if depth % 2 == 0 {
        if node.l != NIL && sx <= x {
            find(node.l, points, nodes, sx, tx, sy, ty, depth + 1, out_ids);
        }
        if node.r != NIL && x <= tx {
            find(node.r, points, nodes, sx, tx, sy, ty, depth + 1, out_ids);
        }
    } else {
        if node.l != NIL && sy <= y {
            find(node.l, points, nodes, sx, tx, sy, ty, depth + 1, out_ids);
        }
        if node.r != NIL && y <= ty {
            find(node.r, points, nodes, sx, tx, sy, ty, depth + 1, out_ids);
        }
    }
}

fn main() {
    // No explicit error handling per spec; unwrap-like behavior is acceptable.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut points: Vec<Point> = Vec::with_capacity(n);
    for i in 0..n {
        let x: i32 = it.next().unwrap().parse().unwrap();
        let y: i32 = it.next().unwrap().parse().unwrap();
        points.push(Point {
            id: i as i32,
            x,
            y,
        });
    }

    let mut nodes: Vec<Node> = Vec::with_capacity(n);
    let root = make_kd_tree(&mut points, &mut nodes, 0, n, 0);

    let q: usize = it.next().unwrap().parse().unwrap();
    let mut output = String::new();
    let mut ids: Vec<i32> = Vec::new();

    for _ in 0..q {
        let sx: i32 = it.next().unwrap().parse().unwrap();
        let tx: i32 = it.next().unwrap().parse().unwrap();
        let sy: i32 = it.next().unwrap().parse().unwrap();
        let ty: i32 = it.next().unwrap().parse().unwrap();

        ids.clear();
        find(root, &points, &nodes, sx, tx, sy, ty, 0, &mut ids);

        ids.sort_unstable_by(|a, b| match a.cmp(b) {
            Ordering::Equal => Ordering::Equal,
            other => other,
        });

        for id in &ids {
            output.push_str(&format!("{id}\n"));
        }
        output.push('\n');
    }

    print!("{output}");
}
