// AOJ 1267: How I Mathematician Wonder What You Are!
// 2017.12.18 bal4u@uu

use std::f64;
use std::io::{self, BufRead};

const EPS: f64 = 1e-8;

#[derive(Clone, Copy)]
struct PP {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy)]
struct LINE {
    s: PP,
    e: PP,
}

static A: [PP; 5] = [
    PP { x: 0.0, y: 0.0 },
    PP { x: 10000.0, y: 0.0 },
    PP { x: 10000.0, y: 10000.0 },
    PP { x: 0.0, y: 10000.0 },
    PP { x: 0.0, y: 0.0 },
];

fn dcmp(x: f64) -> i32 {
    if x.abs() < EPS {
        return 0;
    }
    if x < 0.0 {
        return -1;
    }
    1
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

fn convex_cut(u: &LINE, n: usize, p: &[PP], po: &mut [PP]) -> usize {
    let mut top = 0;

    for i in 0..n {
        let d1 = dcmp(cross(u.e.x - u.s.x, u.e.y - u.s.y, p[i].x - u.s.x, p[i].y - u.s.y));
        let d2 = dcmp(cross(u.e.x - u.s.x, u.e.y - u.s.y, p[i + 1].x - u.s.x, p[i + 1].y - u.s.y));
        if d1 >= 0 {
            po[top] = p[i];
            top += 1;
        }
        if d1 * d2 < 0 {
            line_intersection(&mut po[top], u, &p[i], &p[i + 1]);
            top += 1;
        }
    }
    po[top] = po[0]; // Closing the polygon
    top + 1 // Return the number of points in the new polygon
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut p = vec![PP { x: 0.0, y: 0.0 }; 51];
        for i in 0..n {
            let coords: Vec<i32> = lines.next().unwrap().unwrap().trim().split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
            p[i] = PP { x: coords[0] as f64, y: coords[1] as f64 };
        }
        p[n] = p[0]; // Closing the polygon

        let mut pol = [[PP { x: 0.0, y: 0.0 }; 100]; 2];
        pol[0].copy_from_slice(&A);

        let mut k = 4;
        let mut k1 = 0;
        let mut k2 = 1;

        for i in 0..n {
            let u = LINE { s: p[i], e: p[i + 1] };
            k = convex_cut(&u, k, pol[k1], &mut pol[k2]);
            if k == 0 {
                break;
            }
            std::mem::swap(&mut k1, &mut k2);
        }
        println!("{}", if k > 0 { "1" } else { "0" });
    }
}
