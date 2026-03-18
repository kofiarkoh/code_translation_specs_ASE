use std::io::{self, BufRead};
use std::cmp::Ordering;

fn mizuhasu(x: &i32, y: &i32) -> Ordering {
    if x > y {
        Ordering::Less // Return Less for descending order
    } else if x < y {
        Ordering::Greater // Return Greater for descending order
    } else {
        Ordering::Equal
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let first_line = input.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace().map(|s| s.parse::<i32>());
    let x = params.next().unwrap().unwrap();
    let y = params.next().unwrap().unwrap();
    let a = params.next().unwrap().unwrap();
    let b = params.next().unwrap().unwrap();
    let c = params.next().unwrap().unwrap();

    let mut p: Vec<i32> = Vec::with_capacity(a as usize);
    let mut q: Vec<i32> = Vec::with_capacity(b as usize);
    let mut r: Vec<i32> = Vec::with_capacity(c as usize);

    for _ in 0..a {
        let line = input.next().unwrap().unwrap();
        p.push(line.trim().parse::<i32>().unwrap());
    }
    for _ in 0..b {
        let line = input.next().unwrap().unwrap();
        q.push(line.trim().parse::<i32>().unwrap());
    }
    for _ in 0..c {
        let line = input.next().unwrap().unwrap();
        r.push(line.trim().parse::<i32>().unwrap());
    }

    p.sort_by(mizuhasu);
    q.sort_by(mizuhasu);
    r.sort_by(mizuhasu);

    let mut asumi = x - 1;
    let mut iguti = y - 1;
    let mut kitaeri: i64 = 0;

    for i in 0..c {
        if asumi < 0 && iguti < 0 {
            break;
        } else if asumi < 0 || (iguti >= 0 && p[asumi as usize] > q[iguti as usize]) {
            if r[i as usize] > q[iguti as usize] {
                q[iguti as usize] = r[i as usize];
                iguti -= 1;
            } else {
                break;
            }
        } else { // asumi >= 0 && (iguti < 0 || p[asumi] <= q[iguti])
            if r[i as usize] > p[asumi as usize] {
                p[asumi as usize] = r[i as usize];
                asumi -= 1;
            } else {
                break;
            }
        }
    }

    for i in 0..x {
        kitaeri += p[i as usize] as i64;
    }
    for i in 0..y {
        kitaeri += q[i as usize] as i64;
    }

    println!("{}", kitaeri);
}
