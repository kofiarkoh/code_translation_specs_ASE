use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Point {
    id: usize,
    x: i64,
    y: i64,
}

struct Node {
    point: Point,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn build(mut pts: Vec<Point>, depth: usize) -> Option<Box<Node>> {
        if pts.is_empty() {
            return None;
        }
        let axis = depth & 1; // 0 => x, 1 => y

        // Sort by the chosen axis; tie-break by id for determinism.
        pts.sort_by(|a, b| {
            let primary = if axis == 0 {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            };
            if primary == Ordering::Equal {
                a.id.cmp(&b.id)
            } else {
                primary
            }
        });

        let mid = pts.len() / 2;
        let point = pts[mid];

        let left_vec = pts[..mid].to_vec();
        let right_vec = pts[mid + 1..].to_vec();

        Some(Box::new(Node {
            point,
            left: Node::build(left_vec, depth + 1),
            right: Node::build(right_vec, depth + 1),
        }))
    }

    fn query(&self, sx: i64, tx: i64, sy: i64, ty: i64, depth: usize, out: &mut Vec<usize>) {
        let p = self.point;
        if sx <= p.x && p.x <= tx && sy <= p.y && p.y <= ty {
            out.push(p.id);
        }

        let axis = depth & 1;
        if axis == 0 {
            // Split by x
            if let Some(ref l) = self.left {
                if sx <= p.x {
                    l.query(sx, tx, sy, ty, depth + 1, out);
                }
            }
            if let Some(ref r) = self.right {
                if p.x <= tx {
                    r.query(sx, tx, sy, ty, depth + 1, out);
                }
            }
        } else {
            // Split by y
            if let Some(ref l) = self.left {
                if sy <= p.y {
                    l.query(sx, tx, sy, ty, depth + 1, out);
                }
            }
            if let Some(ref r) = self.right {
                if p.y <= ty {
                    r.query(sx, tx, sy, ty, depth + 1, out);
                }
            }
        }
    }
}

fn main() {
    // No explicit error handling per specification; malformed/missing input => undefined behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut points = Vec::with_capacity(n);
    for id in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        points.push(Point { id, x, y });
    }

    let root = Node::build(points, 0);

    let q: usize = it.next().unwrap().parse().unwrap();
    let mut out_buf = String::new();
    let mut res: Vec<usize> = Vec::new();

    for _ in 0..q {
        let sx: i64 = it.next().unwrap().parse().unwrap();
        let tx: i64 = it.next().unwrap().parse().unwrap();
        let sy: i64 = it.next().unwrap().parse().unwrap();
        let ty: i64 = it.next().unwrap().parse().unwrap();

        res.clear();
        if let Some(ref r) = root {
            r.query(sx, tx, sy, ty, 0, &mut res);
        }
        res.sort_unstable();

        for id in &res {
            out_buf.push_str(&id.to_string());
            out_buf.push('\n');
        }
        out_buf.push('\n');
    }

    print!("{out_buf}");
}
