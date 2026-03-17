use std::io::{self, Write};

fn listen_size() -> (usize, usize) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                return (x, y);
            }
        }
    }
}

fn listen_course(course: &mut Vec<Vec<i32>>, x: usize, y: usize) {
    for j in 0..y {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let values: Vec<i32> = input.trim().split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if values.len() == x {
                for i in 0..x {
                    course[i][j] = values[i];
                }
                break;
            }
        }
    }
}

fn analyze_course(course: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut pattern = vec![vec![0; y + 2]; x];

    for i in 0..x {
        if course[i][0] == 0 {
            pattern[i][0] = 1;
        }
    }

    for j in 0..y - 1 {
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

fn ski_resort() -> i32 {
    let (x, y) = listen_size();
    if x == 0 && y == 0 {
        return 0;
    }

    let mut course = vec![vec![0; y]; x];
    listen_course(&mut course, x, y);
    let sum = analyze_course(&course, x, y);
    println!("{}", sum);
    
    1
}

fn main() {
    while ski_resort() != 0 {}
}
