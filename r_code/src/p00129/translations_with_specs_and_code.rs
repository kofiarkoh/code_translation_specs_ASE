use std::io::{self, BufRead};
use std::f64;

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

fn hitcheck_circle(w: &Wall, q: &Query) -> bool {
    let mut in_count = 0;
    let (minx, maxx) = if q.tx < q.sx { (q.tx, q.sx) } else { (q.sx, q.tx) };
    let (miny, maxy) = if q.ty < q.sy { (q.ty, q.sy) } else { (q.sy, q.ty) };

    if w.x + w.r < minx || maxx < w.x - w.r || w.y + w.r < miny || maxy < w.y - w.r {
        return false;
    }

    if (q.tx - w.x).pow(2) + (q.ty - w.y).pow(2) <= (w.r * w.r) {
        in_count += 1;
    }
    if (q.sx - w.x).pow(2) + (q.sy - w.y).pow(2) <= (w.r * w.r) {
        in_count += 1;
    }

    if in_count == 1 {
        return true;
    }
    if in_count == 2 {
        return false;
    }

    let a = q.ty - q.sy;
    let b = q.sx - q.tx;
    let c = (q.tx - q.sx) * q.ty - (q.ty - q.sy) * q.tx;

    if (a * w.x + b * w.y + c).pow(2) <= (w.r * w.r) * (a.pow(2) + b.pow(2)) {
        let c = -c; // convert a*x + b*y + c = 0 to a*x + b*y = c
        if b == 0 {
            let yy = w.r as f64 * w.r as f64 - ((c as f64) / a as f64 - w.x as f64) * ((c as f64) / a as f64 - w.x as f64);
            if yy < 0.0 {
                return false; // No real intersection
            }
            let y1 = (yy.sqrt() + w.y as f64);
            let y2 = (-yy.sqrt() + w.y as f64);
            return (miny as f64 < y1 + EPS && y1 < maxy as f64 + EPS) || (miny as f64 < y2 + EPS && y2 < maxy as f64 + EPS);
        } else {
            let cb = c as f64 / b as f64;
            let ab = a as f64 / b as f64;
            let x2k = 1.0 + ab * ab;
            let xk = -2.0 * w.x as f64 + 2.0 * (w.y as f64 - cb) * ab;
            let tk = w.x as f64 * w.x as f64 + (w.y as f64 - cb) * (w.y as f64 - cb) - w.r as f64 * w.r as f64;

            let discriminant = xk * xk - 4.0 * x2k * tk;
            if discriminant < 0.0 {
                return false; // No real intersection
            }

            let x1 = (-xk + discriminant.sqrt()) / (2.0 * x2k);
            let x2 = (-xk - discriminant.sqrt()) / (2.0 * x2k);
            return (minx as f64 < x1 + EPS && x1 < maxx as f64 + EPS) || (minx as f64 < x2 + EPS && x2 < maxx as f64 + EPS);
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let n: i32 = line.unwrap().trim().parse().expect("Invalid input");
        if n <= 0 {
            break; // Terminate if non-positive number of walls
        }

        let mut walls = Vec::new();
        for _ in 0..n {
            let wall_input: Vec<i32> = reader.lines().next().unwrap().unwrap().trim().split_whitespace()
                .map(|s| s.parse().expect("Invalid wall input")).collect();
            walls.push(Wall { x: wall_input[0], y: wall_input[1], r: wall_input[2] });
        }

        let m: i32 = reader.lines().next().unwrap().unwrap().trim().parse().expect("Invalid input");
        for _ in 0..m {
            let query_input: Vec<i32> = reader.lines().next().unwrap().unwrap().trim().split_whitespace()
                .map(|s| s.parse().expect("Invalid query input")).collect();
            let query = Query { tx: query_input[0], ty: query_input[1], sx: query_input[2], sy: query_input[3] };

            let mut hidden = false;
            if query.tx != query.sx || query.ty != query.sy {
                for wall in &walls {
                    if hitcheck_circle(wall, &query) {
                        hidden = true;
                        break;
                    }
                }
            }
            println!("{}", if hidden { "Danger" } else { "Safe" });
        }
    }
}
