use std::f64;
use std::io::{self, BufRead};

const EPS: f64 = 1e-7;

#[derive(Debug)]
struct Wall {
    x: i32,
    y: i32,
    r: i32,
}

#[derive(Debug)]
struct Query {
    tx: i32,
    ty: i32,
    sx: i32,
    sy: i32,
}

fn hitcheck_circle(w: &Wall, q: &Query) -> i32 {
    let mut in_count = 0;
    let (minx, maxx) = if q.tx < q.sx { (q.tx, q.sx) } else { (q.sx, q.tx) };
    let (miny, maxy) = if q.ty < q.sy { (q.ty, q.sy) } else { (q.sy, q.ty) };

    if w.x + w.r < minx || maxx < w.x - w.r {
        return 0;
    }
    if w.y + w.r < miny || maxy < w.y - w.r {
        return 0;
    }

    if (q.tx - w.x).pow(2) + (q.ty - w.y).pow(2) <= (w.r * w.r) {
        in_count += 1;
    }
    if (q.sx - w.x).pow(2) + (q.sy - w.y).pow(2) <= (w.r * w.r) {
        in_count += 1;
    }

    if in_count == 1 {
        return 1;
    }
    if in_count == 2 {
        return 0;
    }

    let a = q.ty - q.sy;
    let b = q.sx - q.tx;
    let mut c = (q.tx - q.sx) * q.ty - (q.ty - q.sy) * q.tx;

    if (a * w.x + b * w.y + c).pow(2) <= (w.r * w.r) * (a * a + b * b) {
        c = -c; // convert a*x + b*y + c = 0 to a*x + b*y = c
        if b == 0 {
            let yy = w.r as f64 * w.r as f64 - ((c as f64 / a as f64) - w.x as f64).powi(2);
            if yy < 0.0 {
                return 0;
            }
            let y1 = (yy).sqrt() + w.y as f64;
            let y2 = -((yy).sqrt()) + w.y as f64;
            return ((miny as f64 < y1 + EPS && y1 < maxy as f64 + EPS) ||
                    (miny as f64 < y2 + EPS && y2 < maxy as f64 + EPS)) as i32;
        } else {
            let cb = c as f64 / b as f64;
            let ab = a as f64 / b as f64;
            let x2k = 1.0 + ab * ab;
            let xk = -2.0 * w.x as f64 + 2.0 * (w.y as f64 - cb) * ab;
            let tk = (w.x as f64).powi(2) + (w.y as f64 - cb).powi(2) - (w.r as f64).powi(2);
            let x1 = (-xk + (xk * xk - 4.0 * x2k * tk).sqrt()) / (2.0 * x2k);
            let x2 = (-xk - (xk * xk - 4.0 * x2k * tk).sqrt()) / (2.0 * x2k);
            return ((minx as f64 < x1 + EPS && x1 < maxx as f64 + EPS) ||
                    (minx as f64 < x2 + EPS && x2 < maxx as f64 + EPS)) as i32;
        }
    }
    0
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let n: i32 = line.unwrap().trim().parse().unwrap();
        if n <= 0 {
            break;
        }

        let mut walls = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let x: i32 = parts.next().unwrap().parse().unwrap();
            let y: i32 = parts.next().unwrap().parse().unwrap();
            let r: i32 = parts.next().unwrap().parse().unwrap();
            walls.push(Wall { x, y, r });
        }

        let m: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        for _ in 0..m {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let tx: i32 = parts.next().unwrap().parse().unwrap();
            let ty: i32 = parts.next().unwrap().parse().unwrap();
            let sx: i32 = parts.next().unwrap().parse().unwrap();
            let sy: i32 = parts.next().unwrap().parse().unwrap();
            let q = Query { tx, ty, sx, sy };

            let mut hidden = 0;
            if q.tx != q.sx || q.ty != q.sy {
                for wall in &walls {
                    if hitcheck_circle(wall, &q) != 0 {
                        hidden = 1;
                        break;
                    }
                }
            }
            println!("{}", if hidden != 0 { "Safe" } else { "Danger" });
        }
    }
}
