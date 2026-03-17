use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::ptr;

const MAX_N: usize = 1_000_000;
const NIL: i32 = -1;

#[derive(Debug)]
struct Point {
    id: usize,
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Node {
    location: usize,
    l: i32,
    r: i32,
}

static mut P: [Point; MAX_N] = [Point { id: 0, x: 0, y: 0 }; MAX_N];
static mut T: [Node; MAX_N] = [Node { location: 0, l: NIL, r: NIL }; MAX_N];
static mut NP: usize = 0;
static mut N_PID: usize = 0;
static mut PID: [i32; MAX_N] = [0; MAX_N];

fn cmpx_func(l: &Point, r: &Point) -> Ordering {
    l.x.cmp(&r.x)
}

fn cmpy_func(l: &Point, r: &Point) -> Ordering {
    l.y.cmp(&r.y)
}

fn make_kd_tree(l: usize, r: usize, depth: usize) -> i32 {
    if l >= r {
        return NIL;
    }

    if r - l >= 2 {
        let slice = &mut unsafe { P[l..r] };
        if depth % 2 == 0 {
            slice.sort_by(cmpx_func);
        } else {
            slice.sort_by(cmpy_func);
        }
    }

    let mid = (l + r) / 2;
    let s = unsafe { NP };
    unsafe { NP += 1 };
    unsafe {
        T[s].location = mid;
        T[s].l = make_kd_tree(l, mid, depth + 1);
        T[s].r = make_kd_tree(mid + 1, r, depth + 1);
    }
    s as i32
}

fn find(v: i32, sx: i32, tx: i32, sy: i32, ty: i32, depth: usize) {
    let x = unsafe { P[unsafe { T[v as usize].location] }.x };
    let y = unsafe { P[unsafe { T[v as usize].location] }.y };

    if sx <= x && x <= tx && sy <= y && y <= ty {
        unsafe {
            PID[N_PID] = P[T[v as usize].location].id as i32;
            N_PID += 1;
        }
    }

    if depth % 2 == 0 {
        if unsafe { T[v as usize].l } != NIL {
            if sx <= x {
                find(unsafe { T[v as usize].l }, sx, tx, sy, ty, depth + 1);
            }
        }
        if unsafe { T[v as usize].r } != NIL {
            if x <= tx {
                find(unsafe { T[v as usize].r }, sx, tx, sy, ty, depth + 1);
            }
        }
    } else {
        if unsafe { T[v as usize].l } != NIL {
            if sy <= y {
                find(unsafe { T[v as usize].l }, sx, tx, sy, ty, depth + 1);
            }
        }
        if unsafe { T[v as usize].r } != NIL {
            if y <= ty {
                find(unsafe { T[v as usize].r }, sx, tx, sy, ty, depth + 1);
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut coords = line.split_whitespace();
        let x: i32 = coords.next().unwrap().parse().unwrap();
        let y: i32 = coords.next().unwrap().parse().unwrap();
        unsafe {
            P[i].id = i;
            P[i].x = x;
            P[i].y = y;
            T[i].l = NIL;
            T[i].r = NIL;
        }
    }

    unsafe { NP = 0 };
    let root = make_kd_tree(0, n, 0);

    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut bounds = line.split_whitespace();
        let sx: i32 = bounds.next().unwrap().parse().unwrap();
        let tx: i32 = bounds.next().unwrap().parse().unwrap();
        let sy: i32 = bounds.next().unwrap().parse().unwrap();
        let ty: i32 = bounds.next().unwrap().parse().unwrap();
        
        unsafe { N_PID = 0 };
        find(root, sx, tx, sy, ty, 0);
        
        let mut pid_slice = unsafe { &mut PID[..N_PID] };
        pid_slice.sort();

        for &id in pid_slice {
            println!("{}", id);
        }
        println!();
    }
}
