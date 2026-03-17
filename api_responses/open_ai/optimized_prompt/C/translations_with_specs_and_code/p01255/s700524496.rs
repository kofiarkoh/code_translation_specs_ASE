use std::io::{self, BufRead};
use std::collections::HashSet;

const EPS: f64 = 1e-6;
const INF: f64 = 1e8;
const N: usize = 500;
const MAX: usize = 5000;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Vertex {
    segment: Segment,
    id: usize,
}

#[derive(Debug)]
struct Adjacency {
    hi: usize,
    to: [usize; 25],
    id: [usize; 25],
}

#[derive(Debug)]
struct State {
    v: usize,
    hi: usize,
    hi2: usize,
}

fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

fn cross(a: Point, b: Point) -> f64 {
    a.x * b.y - a.y * b.x
}

fn ccw(p: Point, q: Point, r: Point) -> i32 {
    let dx = q.x - p.x;
    let dy = q.y - p.y;
    let dx_ = r.x - p.x;
    let dy_ = r.y - p.y;
    let t = dx * dy_ - dy * dx_;
    if eq(t, 0.0) {
        return 0;
    }
    if t < 0.0 {
        return -1;
    }
    1
}

fn intersect(s1: Segment, s2: Segment) -> bool {
    ccw(s1.start, s1.end, s2.start) * ccw(s1.start, s1.end, s2.end) <= 0 &&
    ccw(s2.start, s2.end, s1.start) * ccw(s2.start, s2.end, s1.end) <= 0
}

fn cross_point(s1: Segment, s2: Segment) -> (bool, Point) {
    let n = cross(Point { x: s2.start.x - s1.start.x, y: s2.start.y - s1.start.y }, Point { x: s2.end.x - s2.start.x, y: s2.end.y - s2.start.y });
    let d = cross(Point { x: s1.end.x - s1.start.x, y: s1.end.y - s1.start.y }, Point { x: s2.end.x - s2.start.x, y: s2.end.y - s2.start.y });
    if eq(d, 0.0) {
        return (false, Point { x: 1.0, y: 1.0 });
    }
    let intersection = Point {
        x: s1.start.x + (n / d) * (s1.end.x - s1.start.x),
        y: s1.start.y + (n / d) * (s1.end.y - s1.start.y),
    };
    (true, intersection)
}

fn uniq(points: &mut Vec<Point>) -> usize {
    let mut i = 0;
    let mut j = 1;
    while j < points.len() {
        while j < points.len() && eq(points[i].x, points[j].x) && eq(points[i].y, points[j].y) {
            j += 1;
        }
        if j < points.len() {
            if i + 1 != j {
                points[i + 1] = points[j];
            }
            i += 1;
        }
    }
    i + 1
}

fn seg_arrange(data: &mut Vec<Vertex>, n: usize) -> (Vec<Point>, Vec<Adjacency>, Vec<State>) {
    let mut tbl = Vec::new();
    for i in 0..n {
        tbl.push(data[i].segment.start);
        tbl.push(data[i].segment.end);
    }
    let mut sz = tbl.len();
    for i in 0..n {
        for j in 0..i {
            if intersect(data[i].segment, data[j].segment) {
                let (ok, intersection) = cross_point(data[i].segment, data[j].segment);
                if ok {
                    tbl.push(intersection);
                    sz += 1;
                }
            }
        }
    }
    tbl.sort_by(|a, b| {
        if eq(a.x, b.x) {
            if eq(a.y, b.y) {
                std::cmp::Ordering::Equal
            } else if a.y < b.y {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else if a.x < b.x {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let unique_size = uniq(&mut tbl);
    
    let mut aa: Vec<Vec<Adjacency>> = vec![vec![Adjacency { hi: 0, to: [0; 25], id: [0; 25] }; 25]; unique_size];
    let mut ff = vec![0; unique_size];
    let mut u = vec![0; N];
    
    for i in 0..n {
        let mut k = 0;
        for j in 0..unique_size {
            if ccw(data[i].segment.start, data[i].segment.end, tbl[j]) == 0 {
                u[k] = j;
                k += 1;
            }
        }
        for j in 0..k - 1 {
            push_back(&mut aa, u[j], data[i].id, u[j + 1], data[i].id);
            push_back(&mut aa, u[j + 1], data[i].id, u[j], data[i].id);
        }
    }
    
    let mut ss: Vec<State> = (0..unique_size).map(|i| State { v: i, hi: ff[i] }).collect();
    ss.sort_by(|a, b| {
        if a.hi != b.hi {
            b.hi.cmp(&a.hi)
        } else {
            a.v.cmp(&b.v)
        }
    });
    
    (tbl, aa, ss)
}

fn push_back(aa: &mut Vec<Vec<Adjacency>>, i: usize, iid: usize, j: usize, jid: usize) {
    let k = &mut aa[i][iid].hi;
    if !aa[i][iid].to[..*k].iter().any(|&x| x == j && aa[i][iid].id[*k] == jid) {
        if *k == 0 {
            aa[i][iid].hi += 1;
        }
        aa[i][iid].to[*k] = j;
        aa[i][iid].id[*k] = jid;
        *k += 1;
    }
}

fn welsh_powell(aa: &Vec<Vec<Adjacency>>, ss: &Vec<State>, lim: usize) -> usize {
    let mut conflict = vec![vec![0; 25]; 25];
    for i in 0..lim {
        conflict[i][i] = 1;
    }
    for i in 0..ss.len() {
        let u = ss[i].v;
        for c in 0..lim {
            if aa[u][c].hi > 0 {
                for cc in c + 1..lim {
                    if aa[u][cc].hi > 0 {
                        conflict[c][cc] = 1;
                        conflict[cc][c] = 1;
                    }
                }
            }
        }
    }
    
    let mut ans = lim;
    for _ in 0..2 {
        let mut color = vec![0; 25];
        let mut r = 0;
        for i in 0..ss.len() {
            let u = ss[i].v;
            let mut f = 0;
            for c in 0..lim {
                if aa[u][c].hi > 0 && color[c] == 0 {
                    color[c] = r + 1;
                    f += 1;
                    break;
                }
            }
            if f == 0 {
                continue;
            }
            for j in 0..ss.len() {
                let u = ss[j].v;
                for cc in 0..lim {
                    if aa[u][cc].hi > 0 && color[cc] == 0 {
                        for k in 0..f {
                            if conflict[color[k] as usize][cc] == 0 {
                                color[cc] = r + 1;
                                break;
                            }
                        }
                    }
                }
            }
        }
        if r < ans {
            ans = r;
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = reader.lines().filter_map(Result::ok);
    
    while let Some(line) = input.next() {
        let lim: usize = line.trim().parse().unwrap();
        let mut data: Vec<Vertex> = Vec::new();
        let mut n = 0;
        
        for i in 0..lim {
            let s: usize = input.next().unwrap().trim().parse().unwrap();
            if s == 0 {
                continue;
            }
            let start_x: f64 = input.next().unwrap().trim().parse().unwrap();
            let start_y: f64 = input.next().unwrap().trim().parse().unwrap();
            let mut segment = Segment {
                start: Point { x: start_x, y: start_y },
                end: Point { x: 0.0, y: 0.0 },
            };
            for _ in 1..s {
                let end_x: f64 = input.next().unwrap().trim().parse().unwrap();
                let end_y: f64 = input.next().unwrap().trim().parse().unwrap();
                segment.end = Point { x: end_x, y: end_y };
                data.push(Vertex { segment, id: i });
                n += 1;
                segment.start = Point { x: end_x, y: end_y };
            }
        }
        
        let (tbl, aa, ss) = seg_arrange(&mut data, n);
        let result = welsh_powell(&aa, &ss, 25);
        println!("{}", result);
    }
}
