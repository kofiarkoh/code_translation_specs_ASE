use std::io::{self, BufRead};

const EPS: f64 = 1e-8;

#[derive(Clone, Copy)]
struct PP {
    x: f64,
    y: f64,
}

struct LINE {
    s: PP,
    e: PP,
}

fn dcmp(x: f64) -> i32 {
    if x.abs() < EPS {
        0
    } else if x < 0.0 {
        -1
    } else {
        1
    }
}

fn cross(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ax * by - ay * bx
}

fn line_intersection(p: &mut PP, a: &LINE, vs: &PP, ve: &PP) {
    let a1 = cross(ve.x - vs.x, ve.y - vs.y, a.s.x - vs.x, a.s.y - vs.y);
    let a2 = cross(ve.x - vs.x, ve.y - vs.y, a.e.x - vs.x, a.e.y - vs.y);
    p.x = (a.s.x * a2 - a.e.x * a1) / (a2 - a1);
    p.y = (a.s.y * a2 - a.e.y * a1) / (a2 - a1);
}

fn convex_cut(u: &LINE, n: usize, p: &[PP], po: &mut Vec<PP>) -> usize {
    let mut top = 0;

    for i in 0..n {
        let d1 = dcmp(cross(u.e.x - u.s.x, u.e.y - u.s.y, p[i].x - u.s.x, p[i].y - u.s.y));
        let d2 = dcmp(cross(u.e.x - u.s.x, u.e.y - u.s.y, p[i + 1].x - u.s.x, p[i + 1].y - u.s.y));
        
        if d1 >= 0 {
            po.push(p[i]);
            top += 1;
        }
        if d1 * d2 < 0 {
            let mut intersection_point = PP { x: 0.0, y: 0.0 };
            line_intersection(&mut intersection_point, u, &p[i], &p[i + 1]);
            po.push(intersection_point);
            top += 1;
        }
    }
    if top > 0 {
        po.push(po[0]); // Close the polygon
    }
    top
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let a: [PP; 5] = [
        PP { x: 0.0, y: 0.0 },
        PP { x: 10000.0, y: 0.0 },
        PP { x: 10000.0, y: 10000.0 },
        PP { x: 0.0, y: 10000.0 },
        PP { x: 0.0, y: 0.0 },
    ];

    while let Some(line) = lines.next() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut p = vec![PP { x: 0.0, y: 0.0 }; n + 1];
        for i in 0..n {
            let coords: Vec<i32> = lines.next().unwrap().unwrap().trim().split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
            p[i] = PP { x: coords[0] as f64, y: coords[1] as f64 };
        }
        p[n] = p[0]; // Close the polygon

        let mut pol = vec![a.to_vec(), vec![]];
        let mut k = 4;

        for i in 0..n {
            let u = LINE { s: p[i], e: p[i + 1] };
            k = convex_cut(&u, k, &pol[i % 2], &mut pol[(i + 1) % 2]);
            if k == 0 {
                break;
            }
        }
        println!("{}", if k > 0 { 1 } else { 0 });
    }
}
