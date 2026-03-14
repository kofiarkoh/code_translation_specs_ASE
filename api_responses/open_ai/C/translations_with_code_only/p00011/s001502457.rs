use std::io::{self, BufRead};

fn swap(a: &mut i32, b: &mut i32) {
    let t = *a;
    *a = *b;
    *b = t;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first integer w
    let w: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    // Read the second integer n
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Create a vector with length w + 1 to match 1-based indexing
    let mut l = vec![0; (w + 1) as usize];
    for i in 1..=w {
        l[i as usize] = i;
    }

    // Read n pairs of integers and swap their positions in the vector
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        let a: usize = parts[0].trim().parse().unwrap();
        let b: usize = parts[1].trim().parse().unwrap();
        swap(&mut l[a], &mut l[b]);
    }

    // Print the results
    for i in 1..=w {
        println!("{}", l[i as usize]);
    }
}
