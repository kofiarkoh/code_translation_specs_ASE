use std::io::{self, BufRead};
use std::cmp::Ordering;

fn mizuhasu(x: &i32, y: &i32) -> Ordering {
    y.cmp(x) // Sort in descending order
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read X, Y, A, B, C
    let first_line = input.next().unwrap().unwrap();
    let mut numbers: Vec<i32> = first_line.split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    if numbers.len() != 5 {
        std::process::exit(1);
    }

    let (x, y, a, b, c) = (numbers[0], numbers[1], numbers[2], numbers[3], numbers[4]);

    // Read first array
    let mut p: Vec<i32> = input.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    
    if p.len() != a as usize {
        std::process::exit(1);
    }

    // Read second array
    let mut q: Vec<i32> = input.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    
    if q.len() != b as usize {
        std::process::exit(1);
    }

    // Read third array
    let mut r: Vec<i32> = input.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    
    if r.len() != c as usize {
        std::process::exit(1);
    }

    // Sort arrays in descending order
    p.sort_by(mizuhasu);
    q.sort_by(mizuhasu);
    r.sort_by(mizuhasu);

    let mut asumi = (x - 1) as isize; // Index for p
    let mut iguti = (y - 1) as isize; // Index for q

    for &value in &r {
        if asumi < 0 && iguti < 0 {
            break;
        } else if asumi < 0 || (iguti >= 0 && p[asumi as usize] > q[iguti as usize]) {
            if value > q[iguti as usize] {
                q[iguti as usize] = value;
                iguti -= 1;
            } else {
                break;
            }
        } else { // asumi >= 0 && (iguti < 0 || p[asumi] <= q[iguti])
            if value > p[asumi as usize] {
                p[asumi as usize] = value;
                asumi -= 1;
            } else {
                break;
            }
        }
    }

    // Calculate the sum of the first X elements of p and first Y elements of q
    let kitaeri: i64 = p.iter().take(x as usize).sum::<i32>() as i64 + 
                        q.iter().take(y as usize).sum::<i32>() as i64;

    println!("{}", kitaeri);
}
