use std::io::{self, BufRead};

const PRIME: i64 = 10_000_000_019;

fn llswap(a: &mut i64, b: &mut i64) {
    std::mem::swap(a, b);
}

fn llsankaku(x: i64) -> i64 {
    (1 + x) * x / 2
}

fn calcul(mut p: i64, mut q: i64, mut z: i64) -> (i64, i64) {
    let mut f = 0;
    let mut cnt = 0;
    let mut r = 0;
    let mut b = 0;
    let mut i = 0;

    if p < q {
        f = 1;
        llswap(&mut p, &mut q);
    }

    while z >= 0 {
        let w = z / q;
        cnt += w + 1; // Removed unnecessary parentheses
        r += i * (w + 1);
        b += llsankaku(w);
        z -= p;
        i += 1;
    }

    if f == 1 {
        llswap(&mut r, &mut b);
    }

    (cnt, r + b)
}

fn calc(p: i64, q: i64, k: i64) -> (i64, i64) {
    let mut st = 0;
    let mut fi = PRIME * 2000;

    while st <= fi {
        let te = (st + fi) / 2;
        let (c, _) = calcul(p, q, te);
        if c == k {
            return (te, c);
        }
        if c < k {
            st = te + 1;
        } else {
            fi = te - 1;
        }
    }
    (st, fi)
}

#[derive(Clone, Copy)]
struct Point {
    xz: i64,
    yz: i64,
}

fn vector_minus(a: Point, b: Point) -> Point {
    Point {
        xz: a.xz - b.xz,
        yz: a.yz - b.yz,
    }
}

fn crossproduct(a: Point, b: Point) -> i64 {
    a.xz * b.yz - a.yz * b.xz
}

fn is_possible(r: i64, b: i64, k: i64) -> bool {
    let mut st = 1;
    let mut fi = PRIME - 1;
    let mut fr = 0;
    let mut fb = 0;
    let mut sr = 0;
    let mut sb = 0;

    while st <= fi {
        let te = (st + fi) / 2;
        let (zr, zb) = calcul(te, PRIME - te, k);
        if zr <= r && zb <= b {
            return true;
        }
        if zr > r && zb > b {
            return false;
        }
        if zr > r {
            st = te + 1;
            sr = zr;
            sb = zb;
        } else {
            fi = te - 1;
            fr = zr;
            fb = zb;
        }
    }

    let tg = Point { xz: r, yz: b };
    let l1 = Point { xz: fr, yz: fb };
    let l2 = Point { xz: sr, yz: sb };

    crossproduct(vector_minus(l2, l1), vector_minus(tg, l1)) >= 0
}

fn solve(r: i64, b: i64) -> i64 {
    let mut st = 0;
    let mut fi = 2_222_222;

    while st <= fi {
        let te = (st + fi) / 2;
        if is_possible(r, b, te) {
            st = te + 1;
        } else {
            fi = te - 1;
        }
    }
    fi
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let nums: Vec<i64> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let r = nums[0];
    let b = nums[1];
    println!("{}", solve(r, b) - 1); // -1 : empty box
}
