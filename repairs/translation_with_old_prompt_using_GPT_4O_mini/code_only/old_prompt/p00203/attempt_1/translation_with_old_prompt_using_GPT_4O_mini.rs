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

fn listen_size() -> (i32, i32) {
    let mut str = String::new();
    listen(&mut str);
    let mut parts = str.split_whitespace();
    let x = parts.next().and_then(|s| i32::from_str(s).ok()).unwrap_or(0);
    let y = parts.next().and_then(|s| i32::from_str(s).ok()).unwrap_or(0);
    (x, y)
}

fn listen_course(course: &mut Vec<Vec<i32>>, x: usize, y: usize) {
    let mut str = String::new();
    for j in 0..y {
        listen(&mut str);
        let mut parts = str.split_whitespace();
        course[0][j] = parts.next().and_then(|s| i32::from_str(s).ok()).unwrap_or(0);
        for i in 1..x {
            course[i][j] = parts.next().and_then(|s| i32::from_str(s).ok()).unwrap_or(0);
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
            for k in (i as isize - 1)..=(i as isize + 1) {
                if k < 0 || k >= x as isize {
                    continue;
                }
                match course[k as usize][j + 1] {
                    0 => pattern[k as usize][j + 1] += pattern[i][j],
                    1 => pattern[k as usize][j + 1] = 0,
                    _ => {
                        if k as usize == i {
                            let mut m = 0;
                            while (j + 3 + m as usize) < y {
                                if course[k as usize][j + 3 + m as usize] == 2 {
                                    m += 2;
                                } else {
                                    break;
                                }
                            }
                            pattern[k as usize][j + 3 + m as usize] += pattern[i][j];
                        }
                    }
                }
            }
        }
    }
    
    let sum: i32 = pattern.iter().flat_map(|row| &row[y - 1..=y + 1]).sum();
    println!("{}", sum);
}

fn ski_resort() -> bool {
    let (x, y) = listen_size();
    if x == 0 && y == 0 {
        return false;
    }
    
    let mut course = vec![vec![0; y]; x];
    listen_course(&mut course, x as usize, y as usize);
    analyze_course(&course, x as usize, y as usize);
    
    true
}

fn main() {
    while ski_resort() {}
}
