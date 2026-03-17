use std::io::{self, BufRead};
use std::cmp::Ordering;

fn mizuhasu(x: &i32, y: &i32) -> Ordering {
    y.cmp(x) // Sort in descending order
}

fn main() -> Result<(), i32> {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read X, Y, A, B, C
    let first_line = input.next().ok_or(1)??;
    let mut params = first_line.split_whitespace()
        .map(|s| s.parse::<i32>().map_err(|_| 1))
        .collect::<Result<Vec<_>, _>>()?;

    if params.len() != 5 {
        return Err(1);
    }

    let (x, y, a, b, c) = (params[0], params[1], params[2], params[3], params[4]);

    // Validate input sizes
    if x < 0 || y < 0 || a < 0 || b < 0 || c < 0 || a > 114514 || b > 114514 || c > 114514 {
        return Err(1);
    }

    let mut p: Vec<i32> = Vec::with_capacity(a as usize);
    let mut q: Vec<i32> = Vec::with_capacity(b as usize);
    let mut r: Vec<i32> = Vec::with_capacity(c as usize);

    // Read array p
    for _ in 0..a {
        let line = input.next().ok_or(1)??;
        p.push(line.trim().parse::<i32>().map_err(|_| 1)?);
    }

    // Read array q
    for _ in 0..b {
        let line = input.next().ok_or(1)??;
        q.push(line.trim().parse::<i32>().map_err(|_| 1)?);
    }

    // Read array r
    for _ in 0..c {
        let line = input.next().ok_or(1)??;
        r.push(line.trim().parse::<i32>().map_err(|_| 1)?);
    }

    // Sort arrays in descending order
    p.sort_by(mizuhasu);
    q.sort_by(mizuhasu);
    r.sort_by(mizuhasu);

    let mut asumi = (x - 1).max(0);
    let mut iguti = (y - 1).max(0);
    let mut kitaeri: i64 = 0;

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

    // Calculate the sum of retained elements
    for i in 0..x {
        kitaeri += p[i as usize] as i64;
    }
    for i in 0..y {
        kitaeri += q[i as usize] as i64;
    }

    println!("{}", kitaeri);
    Ok(())
}
