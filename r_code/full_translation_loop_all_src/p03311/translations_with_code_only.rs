use std::cmp::Ordering;
use std::io::{self, BufRead};

fn cmpnum(n1: &i64, n2: &i64) -> Ordering {
    n1.cmp(n2)
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements
    let n: i64 = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");
    let mut a: Vec<i64> = Vec::with_capacity(n as usize);

    // Read the elements and adjust them
    for i in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse value");
        a.push(value - (i + 1));
    }

    // Sort the array
    a.sort_by(cmpnum);

    let mut all = 0;
    for i in 1..n {
        all += a[i as usize] - a[0];
    }

    let mut sadness = all;
    let mut min = all;
    let mut level = a[0];
    let mut width: i64;
    let mut height: i64;

    for i in 0..n {
        if i + 1 < n && a[(i + 1) as usize] == level {
            continue;
        }
        width = i + 1;

        if n - width * 2 <= 0 {
            break;
        }

        height = a[(i + 1) as usize] - a[i as usize];

        if height * (n - width * 2) <= sadness {
            level = a[(i + 1) as usize];
            sadness -= height * (n - width * 2);
            min = sadness;
        } else {
            height = sadness / (n - width * 2);
            level += height;
            sadness -= height * (n - width * 2);
            if sadness <= min {
                min = sadness;
            }
            sadness = -(sadness - (n - width * 2));
            if sadness <= min {
                min = sadness;
            }
            break;
        }
    }

    println!("{}", min);
}
