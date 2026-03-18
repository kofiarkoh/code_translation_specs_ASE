use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // undefined by spec; just exit
    };

    let mut a: Vec<i64> = Vec::with_capacity((n as usize) + 1);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }
    // A[N] = A[0]
    a.push(a[0]);

    let mut sum: i64 = 0;
    let mut diff: Vec<i64> = vec![0; (n as usize) + 1]; // indices 0..=N

    for i in 1..=n as usize {
        sum += a[i];
        diff[i] = a[i] - a[i - 1];
    }

    let denom: i64 = n * (n + 1) / 2;
    if sum % denom != 0 {
        print!("NO\n");
        return;
    }

    sum /= denom;

    diff[0] = 0;
    let mut i_idx: usize = 1;
    while i_idx <= n as usize {
        // C does integer division; it does not check divisibility.
        diff[i_idx] = (sum - diff[i_idx]) / n;
        if diff[i_idx] < 0 || diff[i_idx] > sum {
            break;
        } else {
            diff[0] += diff[i_idx];
        }
        i_idx += 1;
    }

    if i_idx <= n as usize || diff[0] != sum {
        print!("NO\n");
        return;
    }

    // In C, they mutate A[N] (which currently equals A[0]) and check it becomes 0.
    // Preserve that exact behavior.
    for i in 1..=n as usize {
        let weight = n - (i as i64) + 1;
        a[n as usize] -= diff[i] * weight;
    }

    if a[n as usize] != 0 {
        // Note: C prints "No\n" (lowercase o) here; preserve exact output.
        print!("No\n");
    } else {
        print!("YES\n");
    }
}
