use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n);

    // Read the integers and adjust them according to their indices
    for i in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value - (i as i64 + 1));
    }

    // Sort the array
    a.sort();

    let mut all = 0;
    for i in 1..n {
        all += a[i] - a[0];
    }

    let mut sadness = all;
    let mut min = all;
    let mut level = a[0];
    let mut width: usize;
    let mut height: i64;

    for i in 0..n {
        if i + 1 < n && a[i + 1] == level {
            continue;
        }
        width = i + 1;
        if n - width * 2 <= 0 {
            break;
        }
        height = a[i + 1] - a[i];

        if height * (n - width * 2) <= sadness {
            level = a[i + 1];
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

    // Output the result
    println!("{}", min);
}
