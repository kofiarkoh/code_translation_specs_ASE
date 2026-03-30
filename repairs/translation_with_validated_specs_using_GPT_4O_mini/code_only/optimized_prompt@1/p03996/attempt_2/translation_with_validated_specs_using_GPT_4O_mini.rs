use std::io::{self, BufRead};
use std::ptr;

#[derive(Debug)]
struct Data {
    prev: *mut Data,
    next: *mut Data,
    id: usize,
    count: i32,
}

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // === input parameter ===
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: i32 = params.next().unwrap().parse().unwrap();
    let m: usize = params.next().unwrap().parse().unwrap();
    let q: usize = params.next().unwrap().parse().unwrap();

    // === operate parameter ===
    let mut a = vec![0; q + 1];

    // === input ===
    for i in 1..=q {
        let line = lines.next().unwrap().unwrap();
        a[i] = line.trim().parse().unwrap();
    }

    // === prepare ===
    let mut d: Vec<Data> = (0..=m)
        .map(|i| Data { prev: ptr::null_mut(), next: ptr::null_mut(), id: i, count: 0 })
        .collect();
    
    let mut p: *mut Data = &mut d[0];

    d[0].count = n;
    for i in (1..=q).rev() {
        let idx = a[i];
        if d[idx].count == 0 {
            unsafe {
                (*d[idx].prev).next = &mut d[idx];
            }
            d[idx].prev = p;
            unsafe {
                (*p).next = &mut d[idx];
            }
            p = &mut d[idx];
        }
        d[idx].count = min(d[idx].count + 1, unsafe { (*d[idx].prev).count });
    }

    // === core ===
    let mut complete = vec![false; m + 1];
    let mut allow = 0;
    let mut current = &mut d[0] as *mut Data;

    while !current.is_null() && unsafe { (*current).count } == n {
        complete[unsafe { (*current).id }] = true;
        while complete[allow as usize] {
            allow += 1;
        }
        current = unsafe { (*current).next };
    }

    while !current.is_null() {
        if unsafe { (*current).id } != allow {
            println!("No");
            return;
        }
        complete[allow as usize] = true;
        while complete[allow as usize] {
            allow += 1;
        }
        current = unsafe { (*current).next };
    }
    println!("Yes");
}
