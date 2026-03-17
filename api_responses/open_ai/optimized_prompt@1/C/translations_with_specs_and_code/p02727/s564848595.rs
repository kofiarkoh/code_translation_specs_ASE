use std::io::{self, BufRead};
use std::cmp::Ordering;

fn mizuhasu(a: &i32, b: &i32) -> Ordering {
    b.cmp(a) // Sort in descending order
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read X, Y, A, B, C
    let first_line = input.next().ok_or("Failed to read input")??;
    let mut nums: Vec<i32> = first_line.split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    if nums.len() != 5 {
        return Err("Expected exactly five integers".into());
    }

    let (x, y, a, b, c) = (nums[0], nums[1], nums[2], nums[3], nums[4]);

    // Read first array
    let mut p: Vec<i32> = input.next().ok_or("Failed to read input")??.split_whitespace()
        .map(|s| s.parse())
        .take(a as usize)
        .collect::<Result<_, _>>()?;

    // Read second array
    let mut q: Vec<i32> = input.next().ok_or("Failed to read input")??.split_whitespace()
        .map(|s| s.parse())
        .take(b as usize)
        .collect::<Result<_, _>>()?;

    // Read third array
    let mut r: Vec<i32> = input.next().ok_or("Failed to read input")??.split_whitespace()
        .map(|s| s.parse())
        .take(c as usize)
        .collect::<Result<_, _>>()?;

    // Sort arrays in descending order
    p.sort_by(mizuhasu);
    q.sort_by(mizuhasu);
    r.sort_by(mizuhasu);

    let mut asumi = (x - 1).max(0);
    let mut iguti = (y - 1).max(0);

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

    // Calculate the sum of the first X and Y integers
    let kitaeri: i64 = p.iter().take(x as usize).sum::<i32>() as i64
        + q.iter().take(y as usize).sum::<i32>() as i64;

    println!("{}", kitaeri);
    Ok(())
}
