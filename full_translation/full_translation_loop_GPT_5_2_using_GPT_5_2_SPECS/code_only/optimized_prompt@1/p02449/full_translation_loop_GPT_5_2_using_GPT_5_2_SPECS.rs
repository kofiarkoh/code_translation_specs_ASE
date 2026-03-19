use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // No input: C would have undefined behavior; we just exit.
    };

    // C uses fixed-size arrays of length 9; preserve that.
    let mut a = [0i32; 9];
    let mut next_a = [0i32; 9];
    let mut prev_a = [0i32; 9];

    // Fill first n elements.
    // Note: If n > 9, the C code would write out of bounds (UB). We do not emulate UB;
    // we keep the same storage size and will panic if n > 9 due to bounds checks.
    for i in 0..(n as usize) {
        let v: i32 = it.next().unwrap().parse().unwrap();
        a[i] = v;
        next_a[i] = v;
        prev_a[i] = v;
    }

    let mut out = String::new();

    if n == 1 {
        out.push_str("1\n");
        print!("{out}");
        return;
    }

    // search for previous permutation
    let mut flag = 0i32;
    let mut i_idx: i32 = 0;
    for i in (0..=(n - 2)).rev() {
        let iu = i as usize;
        if prev_a[iu] > prev_a[iu + 1] {
            flag = 1;
            i_idx = i;
            break;
        }
    }

    if flag != 0 {
        for j in ((i_idx + 1)..=n - 1).rev() {
            let ju = j as usize;
            let iu = i_idx as usize;
            if prev_a[ju] < prev_a[iu] {
                let tmp = prev_a[ju];
                prev_a[ju] = prev_a[iu];
                prev_a[iu] = tmp;
                break;
            }
        }
        let mid: i32 = (n - i_idx - 1) / 2;
        for k in 1..=mid {
            let left = (i_idx + k) as usize;
            let right = (n - k) as usize;
            let tmp = prev_a[left];
            prev_a[left] = prev_a[right];
            prev_a[right] = tmp;
        }

        // print the previous permutation
        out.push_str(&format!("{}", prev_a[0]));
        for i in 1..(n as usize) {
            out.push_str(&format!(" {}", prev_a[i]));
        }
        out.push('\n');
    }

    // print the given sequence
    out.push_str(&format!("{}", a[0]));
    for i in 1..(n as usize) {
        out.push_str(&format!(" {}", a[i]));
    }
    out.push('\n');

    // search for next permutation
    flag = 0;
    i_idx = 0;
    for i in (0..=(n - 2)).rev() {
        let iu = i as usize;
        if next_a[iu] < next_a[iu + 1] {
            flag = 1;
            i_idx = i;
            break;
        }
    }

    if flag != 0 {
        for j in ((i_idx + 1)..=n - 1).rev() {
            let ju = j as usize;
            let iu = i_idx as usize;
            if next_a[ju] > next_a[iu] {
                let tmp = next_a[ju];
                next_a[ju] = next_a[iu];
                next_a[iu] = tmp;
                break;
            }
        }
        let mid: i32 = (n - i_idx - 1) / 2;
        for k in 1..=mid {
            let left = (i_idx + k) as usize;
            let right = (n - k) as usize;
            let tmp = next_a[left];
            next_a[left] = next_a[right];
            next_a[right] = tmp;
        }

        // print the next permutation
        out.push_str(&format!("{}", next_a[0]));
        for i in 1..(n as usize) {
            out.push_str(&format!(" {}", next_a[i]));
        }
        out.push('\n');
    }

    print!("{out}");
}
