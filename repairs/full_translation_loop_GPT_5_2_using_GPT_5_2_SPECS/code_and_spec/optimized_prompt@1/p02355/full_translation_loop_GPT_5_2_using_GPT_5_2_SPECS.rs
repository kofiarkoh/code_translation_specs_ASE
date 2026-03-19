use std::io::{self, Read};

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: usize = it.next().unwrap().parse().unwrap();

    // Spec postconditions
    if k == 0 || n < k {
        print!("0\n");
        return;
    }

    let mut a = vec![0usize; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse::<usize>().unwrap();
    }

    // Sliding window with counts for values 1..=k.
    // Values outside this range are irrelevant (spec invariant).
    let mut f = vec![0u32; k + 1]; // index 0 unused
    let mut s: usize = 0; // number of distinct required values currently present
    let mut i: usize = 0;
    let mut j: usize = 0;

    // Expand until we have all 1..=k or reach end.
    while s < k && j < n {
        let t = a[j];
        j += 1;
        if t >= 1 && t <= k {
            if f[t] == 0 {
                s += 1;
            }
            f[t] += 1;
        }
    }

    if s < k {
        print!("0\n");
        return;
    }

    // Shrink from left while leftmost is irrelevant or redundant.
    while i < j {
        let t = a[i];
        if t < 1 || t > k {
            i += 1;
        } else if f[t] > 1 {
            f[t] -= 1;
            i += 1;
        } else {
            break;
        }
    }

    let mut ans = j - i;

    // Continue sliding
    while j < n {
        let t = a[j];
        j += 1;
        if t >= 1 && t <= k {
            f[t] += 1;
        }

        while i < j {
            let t2 = a[i];
            if t2 < 1 || t2 > k {
                i += 1;
            } else if f[t2] > 1 {
                f[t2] -= 1;
                i += 1;
            } else {
                break;
            }
        }

        let len = j - i;
        if len < ans {
            ans = len;
        }
    }

    print!("{ans}\n");
}
