use std::io::{self, BufRead};

#[derive(Debug)]
struct Data {
    prev: Option<*mut Data>,
    next: Option<*mut Data>,
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
        .map(|i| Data { prev: None, next: None, id: i, count: if i == 0 { n } else { 0 } })
        .collect();

    let mut p: *mut Data = &mut d[0];
    for i in (1..=q).rev() {
        if d[a[i]].count == 0 {
            unsafe {
                if let Some(prev) = d[a[i]].prev {
                    (*prev).next = Some(&mut d[a[i]]);
                }
            }
            d[a[i]].prev = Some(p);
            p = &mut d[a[i]];
        }
        d[a[i]].count = min(d[a[i]].count + 1, unsafe { (*d[a[i]].prev.unwrap()).count });
    }

    // === core ===
    let mut complete = vec![false; m + 1];
    let mut allow = 0;
    let mut current: *mut Data = &mut d[0];

    while !current.is_null() && unsafe { (*current).count } == n {
        complete[unsafe { (*current).id }] = true;
        while complete[allow as usize] {
            allow += 1;
        }
        current = unsafe { (*current).next.unwrap_or(std::ptr::null_mut()) };
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
        current = unsafe { (*current).next.unwrap_or(std::ptr::null_mut()) };
    }
    println!("Yes");
}
