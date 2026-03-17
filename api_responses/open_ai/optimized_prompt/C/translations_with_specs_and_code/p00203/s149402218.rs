use std::io::{self, BufRead};

fn listen() -> String {
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        let line = line.unwrap();
        if !line.trim().is_empty() {
            return line;
        }
    }
    String::new() // Should not reach here as per the spec
}

fn listen_size() -> (usize, usize) {
    let input = listen();
    let mut parts = input.split_whitespace();
    let x = parts.next().unwrap_or("0").parse::<usize>().unwrap_or(0);
    let y = parts.next().unwrap_or("0").parse::<usize>().unwrap_or(0);
    (x, y)
}

fn listen_course(x: usize, y: usize) -> Vec<Vec<i32>> {
    let mut course = vec![vec![0; y]; x];
    for j in 0..y {
        let input = listen();
        let mut parts = input.split_whitespace();
        for i in 0..x {
            course[i][j] = parts.next().unwrap_or("0").parse::<i32>().unwrap_or(0);
        }
    }
    course
}

fn analyze_course(course: &[Vec<i32>], x: usize, y: usize) -> i32 {
    let mut pattern = vec![vec![0; y + 2]; x];

    for i in 0..x {
        if course[i][0] == 0 {
            pattern[i][0] = 1;
        }
    }

    for j in 0..(y - 1) {
        for i in 0..x {
            for k in (i.saturating_sub(1))..=(i + 1).min(x - 1) {
                match course[k][j + 1] {
                    0 => pattern[k][j + 1] += pattern[i][j],
                    1 => pattern[k][j + 1] = 0,
                    _ => {
                        if k == i {
                            let mut m = 0;
                            while j + 3 + m < y {
                                if course[k][j + 3 + m] == 2 {
                                    m += 2;
                                } else {
                                    break;
                                }
                            }
                            pattern[k][j + 3 + m] += pattern[i][j];
                        }
                    }
                }
            }
        }
    }

    pattern.iter().flat_map(|row| row[y - 1..=y + 1].iter()).sum()
}

fn ski_resort() -> bool {
    let (x, y) = listen_size();
    if x == 0 && y == 0 {
        return false;
    }
    let course = listen_course(x, y);
    let sum = analyze_course(&course, x, y);
    println!("{}", sum);
    true
}

fn main() {
    while ski_resort() {}
}
