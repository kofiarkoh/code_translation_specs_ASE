use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse two whitespace-delimited strings.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let s1 = it.next().unwrap();
    let s2 = it.next().unwrap();

    // The C code operates on bytes (char), so we do the same.
    let a = s1.as_bytes();
    let b = s2.as_bytes();
    let n = a.len();
    let m = b.len();

    // DP table arr with dimensions (n+1) x (m+1), initialized to 0.
    // Use a flat Vec for contiguous storage.
    let mut arr = vec![0i32; (n + 1) * (m + 1)];
    let idx = |i: usize, j: usize, m: usize| -> usize { i * (m + 1) + j };

    for i in 1..=n {
        for j in 1..=m {
            let v = if a[i - 1] == b[j - 1] {
                arr[idx(i - 1, j - 1, m)] + 1
            } else {
                let up = arr[idx(i - 1, j, m)];
                let left = arr[idx(i, j - 1, m)];
                if up >= left { up } else { left } // matches C's max with >= tie-break
            };
            arr[idx(i, j, m)] = v;
        }
    }

    let len = arr[idx(n, m, m)] as usize;

    // Reconstruct one LCS deterministically, matching the C backtracking:
    // prefer moving up when arr[r][c] == arr[r-1][c], else move left.
    let mut ans = vec![0u8; len];
    let mut ind: isize = len as isize - 1;
    let mut r = n;
    let mut c = m;

    while r >= 1 && c >= 1 {
        if a[r - 1] == b[c - 1] {
            if ind >= 0 {
                ans[ind as usize] = a[r - 1];
            }
            r -= 1;
            c -= 1;
            ind -= 1;
        } else if arr[idx(r, c, m)] == arr[idx(r - 1, c, m)] {
            r -= 1;
        } else {
            c -= 1;
        }
    }

    // Output with no separators and no trailing newline, as specified.
    // If len == 0, prints nothing.
    print!("{}", String::from_utf8_lossy(&ans));
}
