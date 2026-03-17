// AOJ 1064: Yanagi's Comic
// Translated from C to Rust

use std::io::{self, BufRead};

#[derive(Debug)]
struct Rectangle {
    id: usize,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    used: bool,
}

fn read_input() -> Vec<Rectangle> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut rectangles = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: usize = line.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        for i in 0..n {
            let coords: Vec<i32> = reader
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let (x1, y1, x2, y2) = (coords[0], coords[1], coords[2], coords[3]);
            let (x1, x2) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            let (y1, y2) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

            rectangles.push(Rectangle {
                id: i,
                x1,
                y1,
                x2,
                y2,
                used: false,
            });
        }

        let mut order = 0;
        let mut ans = vec![0; n];
        calc(&mut rectangles, &mut ans, &mut order, n);
        
        if !rectangles.is_empty() {
            println!();
        }
        for i in 0..n {
            println!("{}", ans[i]);
        }
    }
    rectangles
}

fn calc(rectangles: &mut Vec<Rectangle>, ans: &mut Vec<usize>, order: &mut usize, n: usize) {
    rectangles.sort_by(|a, b| {
        if a.x1 != b.x1 {
            a.x1.cmp(&b.x1)
        } else {
            a.y1.cmp(&b.y1)
        }
    });

    ans[rectangles[0].id] = *order + 1;
    rectangles[0].used = true;
    *order += 1;

    let y = rectangles[0].y2;
    let mut x = 501;

    for i in 0..n {
        if rectangles[i].used {
            continue;
        }
        if rectangles[i].y1 < y && y < rectangles[i].y2 {
            x = rectangles[i].x1;
            break;
        }
    }

    let mut tmp = Vec::new();
    for i in 0..n {
        if rectangles[i].used {
            continue;
        }
        if rectangles[i].y2 <= y && rectangles[i].x2 <= x {
            tmp.push(rectangles[i].clone());
            rectangles[i].used = true;
        }
    }

    if !tmp.is_empty() {
        calc(&mut tmp, ans, order, tmp.len());
    }

    for i in 0..n {
        if !rectangles[i].used {
            calc(&mut rectangles[i..].to_vec(), ans, order, n - i);
            break;
        }
    }
}

fn main() {
    read_input();
}
