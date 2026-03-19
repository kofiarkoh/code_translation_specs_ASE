use std::io::{self, BufRead};

fn main() {
    while ski_resort() {}
}

fn ski_resort() -> bool {
    let (x, y) = listen_size();
    if x == 0 && y == 0 {
        return false;
    }

    let mut course = vec![vec![0; y]; x];
    listen_course(&mut course, x, y);
    analyze_course(&course, x, y);

    true
}

fn listen_size() -> (usize, usize) {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();
    
    loop {
        line.clear();
        reader.read_line(&mut line).expect("Failed to read line");
        if !line.trim().is_empty() {
            break;
        }
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    let x = parts[0].parse::<usize>().unwrap_or(0);
    let y = parts[1].parse::<usize>().unwrap_or(0);
    (x, y)
}

fn listen_course(course: &mut Vec<Vec<i32>>, x: usize, y: usize) {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    
    for j in 0..x {
        let mut line = String::new();
        reader.read_line(&mut line).expect("Failed to read line");
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        
        for i in 0..y {
            course[j][i] = parts[i];
        }
    }
}

fn analyze_course(course: &Vec<Vec<i32>>, x: usize, y: usize) {
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
                            if j + 3 + m < y {
                                pattern[k][j + 3 + m] += pattern[i][j];
                            }
                        }
                    }
                }
            }
        }
    }

    let sum: i32 = pattern.iter()
        .map(|row| row[y - 1] + row[y] + row[y + 1])
        .sum();
    
    println!("{}", sum);
}
