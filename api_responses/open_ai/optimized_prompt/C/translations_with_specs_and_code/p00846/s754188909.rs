// AOJ 1267: How I Mathematician Wonder What You Are!
// Translated from C to Rust

use std::io::{self, BufRead};
use std::f64;

const EPS: f64 = 1e-8;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

struct Line {
    s: Point,
    e: Point,
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

fn line_intersection(p: &mut Point, a: &Line, vs: &Point, ve: &Point) {
    let a1 = cross(ve.x - vs.x, ve.y - vs.y, a.s.x - vs.x, a.s.y - vs.y);
    let a2 = cross(ve.x - vs.x, ve.y - vs.y, a.e.x - vs.x, a.e.y - vs.y);
    p.x = (a.s.x * a2 - a.e.x * a1) / (a2 - a1);
    p.y = (a.s.y * a2 - a.e.y * a1) / (a2 - a1);
}

fn convex_cut(u: &Line, n: usize, p: &[Point], po: &mut Vec<Point>) -> usize {
    let mut top = 0;

    for i in 0..n {
        let d1 = dcmp(cross(u.e.x - u.s.x, u.e.y - u.s.y, p[i].x - u.s.x, p[i].y - u.s.y));
        let d2 = dcmp(cross(u.e.x - u.s.x, u.e.y - u.s.y, p[i + 1].x - u.s.x, p[i + 1].y - u.s.y));
        
        if d1 >= 0 {
            po.push(p[i]);
            top += 1;
        }
        if d1 * d2 < 0 {
            let mut intersection_point = Point { x: 0.0, y: 0.0 };
            line_intersection(&mut intersection_point, u, &p[i], &p[i + 1]);
            po.push(intersection_point);
            top += 1;
        }
    }
    po.push(po[0]); // Closing the polygon
    top
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = reader.lines();

    let a: [Point; 5] = [
        Point { x: 0.0, y: 0.0 },
        Point { x: 10000.0, y: 0.0 },
        Point { x: 10000.0, y: 10000.0 },
        Point { x: 0.0, y: 10000.0 },
        Point { x: 0.0, y: 0.0 },
    ];

    while let Some(line) = input.next() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            return; // Terminate if n is zero or negative
        }

        let mut p: Vec<Point> = Vec::with_capacity(n + 1);
        for _ in 0..n {
            let coords: Vec<i32> = input.next().unwrap().trim().split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
            p.push(Point { x: coords[0] as f64, y: coords[1] as f64 });
        }
        p.push(p[0]); // Closing the polygon

        let mut pol = vec![vec![Point { x: 0.0, y: 0.0 }; 100]; 2];
        pol[0].clone_from_slice(&a);

        let mut k = 4;
        let mut k1 = 0;
        let mut k2 = 1;

        for i in 0..n {
            let u = Line { s: p[i], e: p[i + 1] };
            let mut po = Vec::new();
            k = convex_cut(&u, k, &pol[k1], &mut po);
            if k == 0 {
                break;
            }
            k1 = k2;
            k2 = 1 - k2; // Toggle between 0 and 1
        }
        println!("{}", if k > 0 { "1" } else { "0" });
    }
}
