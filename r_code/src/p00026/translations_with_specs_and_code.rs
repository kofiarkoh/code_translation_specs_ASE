use std::io::{self, Read};

fn main() {
    // 10x10 grid initialized to zeros
    let mut map: [[i32; 10]; 10] = [[0; 10]; 10];

    // Read entire stdin (input assumed well-formed as "%d,%d,%d" per spec)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdin read failed");

    for line in input.lines() {
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        // Expect exactly three integers separated by commas.
        // Per specification, input is well-formed, so unwraps are acceptable here.
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            // Spec says malformed input is undefined; here we simply skip such lines.
            continue;
        }
        let x: i32 = parts[0].trim().parse().expect("invalid integer");
        let y: i32 = parts[1].trim().parse().expect("invalid integer");
        let level: i32 = parts[2].trim().parse().expect("invalid integer");

        apply_taint(&mut map, x, y, level);
    }

    let zeros = count_zeros(&map);
    let maximum = max_value(&map);

    println!("{}", zeros);
    println!("{}", maximum);
}

// Apply taint according to the fall-through rules:
// s=3 applies taint_3, taint_2, taint_1
// s=2 applies taint_2, taint_1
// s=1 applies taint_1
fn apply_taint(map: &mut [[i32; 10]; 10], x: i32, y: i32, s: i32) {
    match s {
        3 => {
            taint_3(map, x, y);
            taint_2(map, x, y);
            taint_1(map, x, y);
        }
        2 => {
            taint_2(map, x, y);
            taint_1(map, x, y);
        }
        1 => {
            taint_1(map, x, y);
        }
        _ => {
            // s outside {1,2,3} produces no effect
        }
    }
}

// Orthogonal neighbors at Manhattan distance 1
fn taint_1(map: &mut [[i32; 10]; 10], x: i32, y: i32) {
    let candidates = [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
    ];
    for &(rx, ry) in &candidates {
        if in_bounds(rx, ry) {
            map[rx as usize][ry as usize] += 1;
        }
    }
}

// Diagonal neighbors (x±1, y±1)
fn taint_2(map: &mut [[i32; 10]; 10], x: i32, y: i32) {
    let candidates = [
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ];
    for &(rx, ry) in &candidates {
        if in_bounds(rx, ry) {
            map[rx as usize][ry as usize] += 1;
        }
    }
}

// Orthogonal neighbors at Manhattan distance 2
fn taint_3(map: &mut [[i32; 10]; 10], x: i32, y: i32) {
    let candidates = [
        (x - 2, y),
        (x + 2, y),
        (x, y - 2),
        (x, y + 2),
    ];
    for &(rx, ry) in &candidates {
        if in_bounds(rx, ry) {
            map[rx as usize][ry as usize] += 1;
        }
    }
}

fn in_bounds(x: i32, y: i32) -> bool {
    (0..10).contains(&x) && (0..10).contains(&y)
}

fn count_zeros(map: &[[i32; 10]; 10]) -> i32 {
    let mut cnt = 0;
    for row in map.iter() {
        for &v in row.iter() {
            if v == 0 {
                cnt += 1;
            }
        }
    }
    cnt
}

fn max_value(map: &[[i32; 10]; 10]) -> i32 {
    let mut m = 0;
    for row in map.iter() {
        for &v in row.iter() {
            if v > m {
                m = v;
            }
        }
    }
    m
}
