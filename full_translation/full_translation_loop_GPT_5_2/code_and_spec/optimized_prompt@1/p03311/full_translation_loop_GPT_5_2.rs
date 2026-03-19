use std::io::{self, Read};

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let nn = n as usize;

    let mut a: Vec<i64> = Vec::with_capacity(nn);
    for i in 0..nn {
        let mut v: i64 = it.next().unwrap().parse().unwrap();
        v -= (i as i64) + 1;
        a.push(v);
    }

    a.sort();

    // all = sum_{i=1..n-1} (a[i] - a[0])
    let mut all: i64 = 0;
    for i in 1..nn {
        all += a[i] - a[0];
    }

    let mut sadness: i64 = all;
    let mut minv: i64 = all;
    let mut level: i64 = a[0];

    // Note: The C code reads a[i+1] even when i == n-1 (out-of-bounds).
    // That is undefined behavior in C. To preserve intended semantics safely,
    // we iterate only while i+1 < n.
    for i in 0..(nn.saturating_sub(1)) {
        if a[i + 1] == level {
            continue;
        }

        let width: i64 = (i as i64) + 1;
        let denom: i64 = n - width * 2;
        if denom <= 0 {
            break;
        }

        let mut height: i64 = a[i + 1] - a[i];
        let cost: i64 = height * denom;

        if cost <= sadness {
            level = a[i + 1];
            sadness -= cost;
            minv = sadness;
        } else {
            height = sadness / denom;
            level += height;
            sadness -= height * denom;

            if sadness <= minv {
                minv = sadness;
            }

            // sadness = -(sadness - denom)  <=>  sadness = denom - sadness
            sadness = denom - sadness;
            if sadness <= minv {
                minv = sadness;
            }
            break;
        }
    }

    println!("{minv}");
}
