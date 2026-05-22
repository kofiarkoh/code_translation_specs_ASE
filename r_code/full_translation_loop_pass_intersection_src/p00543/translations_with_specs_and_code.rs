use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: i32 = it.next().unwrap().parse().unwrap();

    // C code uses 1-based indexing into a[1000]; we mirror that with a[0] unused.
    let mut a: Vec<i32> = vec![0; n + 1];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    // Passes for k = 1..=m, each pass is exactly one left-to-right sweep.
    for k in 1..=m {
        // Adjacent comparisons: (1,2), (2,3), ..., (n-1,n)
        for i in 1..n {
            let j = i + 1;
            if a[i] % k > a[j] % k {
                a.swap(i, j);
            }
        }
    }

    // Output: n integers, one per line.
    let mut out = String::new();
    for i in 1..=n {
        out.push_str(&format!("{}\n", a[i]));
    }
    print!("{out}");
}
