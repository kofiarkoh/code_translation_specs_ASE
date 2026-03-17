use std::io::{self, BufRead};
use std::str::FromStr;

fn listen(str: &mut String) {
    loop {
        let mut buffer = String::new();
        io::stdin().lock().read_line(&mut buffer).unwrap();
        if buffer.trim() != "" {
            *str = buffer;
            break;
        }
    }
}

fn listen_size() -> (usize, usize) {
    let mut str = String::new();
    listen(&mut str);
    let mut iter = str.split_whitespace();
    let x = iter.next().and_then(|s| usize::from_str(s).ok()).unwrap_or(0);
    let y = iter.next().and_then(|s| usize::from_str(s).ok()).unwrap_or(0);
    (x, y)
}

fn listen_course(course: &mut Vec<Vec<i32>>, x: usize, y: usize) {
    let mut str = String::new();
    for i in 0..x {
        listen(&mut str);
        let mut iter = str.split_whitespace();
        for j in 0..y {
            course[i][j] = iter.next().and_then(|s| i32::from_str(s).ok()).unwrap_or(0);
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

    let sum: i32 = pattern.iter().flat_map(|row| &row[y - 1..=y]).sum();
    println!("{}", sum);
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

fn main() {
    while ski_resort() {}
}
