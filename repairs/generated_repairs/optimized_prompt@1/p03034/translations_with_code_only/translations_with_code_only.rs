use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse number of elements");
    let mut a = vec![0i64; n]; // Adjust size based on n

    // Read the elements into the array
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse element");
    }

    let mut ans = 0;

    // Iterate over possible values of d
    for d in 1..(n - 1) / 2 + 1 { // Adjust loop condition to include the last valid d
        let mut s = 0;
        let mut g = n - 1;
        let mut crr = 0;

        loop {
            let ns = s + d;
            let ng = g - d;

            // Break conditions
            if ng <= d || ns >= ng || ns == g {
                break;
            }

            s = ns;
            g = ng;
            crr += a[s] + a[g];
            ans = ans.max(crr);
        }
    }

    // Print the result
    println!("{}", ans);
}
