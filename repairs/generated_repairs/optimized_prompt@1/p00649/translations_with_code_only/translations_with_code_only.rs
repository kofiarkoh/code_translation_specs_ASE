// AOJ 1064: Yanagi's Comic
// 2018.1.18 bal4u

use std::io::{self, BufRead, Read};

#[derive(Clone)]
struct T {
    id: usize,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    used: bool,
}

static mut ORDER: usize = 0; // Using unsafe to maintain global state similar to C
static mut ANS: [usize; 10] = [0; 10];

fn in_number() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut n = 0;
    let mut c = handle.bytes().next().unwrap().unwrap() as char;

    while c.is_digit(10) {
        n = (n << 3) + (n << 1) + (c as i32 - '0' as i32);
        c = handle.bytes().next().unwrap().unwrap() as char;
    }
    n
}

fn cmp(a: &T, b: &T) -> std::cmp::Ordering {
    if a.x1 != b.x1 {
        return a.x1.cmp(&b.x1);
    }
    a.y1.cmp(&b.y1)
}

fn calc(t: &mut [T], n: usize) {
    let mut tmp: Vec<T> = Vec::new();
    
    t.sort_by(cmp);
    unsafe {
        ANS[t[0].id] = ORDER + 1;
        ORDER += 1;
    }
    t[0].used = true;

    let mut y = t[0].y2;
    let mut x = 501;

    for i in 0..n {
        if t[i].used { continue; }
        if t[i].y1 < y && y < t[i].y2 {
            x = t[i].x1;
            break;
        }
    }

    let mut j = 0;
    for i in 0..n {
        if t[i].used { continue; }
        if t[i].y2 <= y && t[i].x2 <= x {
            tmp.push(t[i].clone());
            t[i].used = true;
            j += 1;
        }
    }
    if j > 0 {
        calc(&mut tmp, j);
    }

    for i in 0..n {
        if !t[i].used {
            calc(&mut t[i..], n - i);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut cno = 0;

    loop {
        let n = in_number();
        if n == 0 { break; }

        let mut tbl: Vec<T> = Vec::with_capacity(n as usize);
        for i in 0..n {
            let x1 = in_number();
            let y1 = in_number();
            let x2 = in_number();
            let y2 = in_number();

            let (x1, x2) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            let (y1, y2) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

            tbl.push(T { id: i as usize, x1, y1, x2, y2, used: false });
        }

        unsafe { ORDER = 0; }
        calc(&mut tbl, n as usize);

        if cno == 0 {
            cno = 1;
        } else {
            println!(); // Print a newline between outputs
        }
        for i in 0..n {
            unsafe { println!("{}", ANS[i as usize]); }
        }
    }
}
