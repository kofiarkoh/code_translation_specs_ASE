use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let h: usize = dimensions.next().unwrap().parse().unwrap();
    let w: usize = dimensions.next().unwrap().parse().unwrap();

    let mut s = vec![String::new(); h];

    for i in 0..h {
        s[i] = lines.next().unwrap().unwrap();
    }

    for i in 0..h {
        for j in 0..w {
            let u = if i > 0 { s[i - 1].chars().nth(j).unwrap() } else { '\0' };
            let l = if j > 0 { s[i].chars().nth(j - 1).unwrap() } else { '\0' };
            let d = if i + 1 < h { s[i + 1].chars().nth(j).unwrap() } else { '\0' };
            let r = if j + 1 < w { s[i].chars().nth(j + 1).unwrap() } else { '\0' };

            if s[i].chars().nth(j).unwrap() == '#' {
                if u == '#' || d == '#' || r == '#' || l == '#' {
                    continue;
                } else {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
