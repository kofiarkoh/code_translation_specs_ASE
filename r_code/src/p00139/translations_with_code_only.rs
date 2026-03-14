use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        let str = lines.next().unwrap().unwrap();
        let mut cnt1 = 0;
        let mut cnt2 = 0;
        let mut flag = 0;

        if str.chars().nth(0) == Some('>') {
            if str.chars().nth(1) == Some('\'') {
                let mut j = 2;
                while j < str.len() {
                    if str.chars().nth(j) == Some('=') {
                        cnt1 += 1;
                    }
                    if j + 1 < str.len() && str.chars().nth(j + 1) == Some('#') {
                        break;
                    }
                    j += 1;
                }
                for j in (cnt1 + 2)..str.len() {
                    if str.chars().nth(j) == Some('=') {
                        cnt2 += 1;
                    }
                }
                if cnt1 == cnt2 && str.chars().nth(cnt1 + cnt2 + 3) == Some('~') && str.chars().nth(cnt1 + cnt2 + 4) == None {
                    println!("A");
                    flag = 1;
                }
            } else if str.chars().nth(1) == Some('^') {
                let mut j = 2;
                while j < str.len() {
                    if j % 2 == 0 {
                        if str.chars().nth(j) == Some('Q') && str.chars().nth(j + 1) == Some('=') {
                            if str.chars().nth(j + 2) == Some('~') && str.chars().nth(j + 3) == Some('~') && j + 4 == str.len() {
                                println!("B");
                                flag = 1;
                            }
                        } else {
                            break;
                        }
                    }
                    j += 1;
                }
            }
        }
        if flag == 0 {
            println!("NA");
        }
    }
}
