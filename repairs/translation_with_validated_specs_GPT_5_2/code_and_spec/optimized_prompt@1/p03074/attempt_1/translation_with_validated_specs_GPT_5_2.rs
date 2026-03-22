use std::io::{self, Read};

fn main() {
    // Read entire stdin, split by whitespace (matches scanf("%d %d") then scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap(); // precondition: at least N binary chars present

    // Convert first N chars to boolean bits: '1' => true, else false.
    // Then append a sentinel bit that differs from the last bit to flush the final run.
    let mut bits: Vec<bool> = Vec::with_capacity(n + 1);
    for b in s.as_bytes().iter().take(n) {
        bits.push(*b == b'1');
    }
    // Sentinel: opposite of last bit (C code assumes n>0 per spec).
    bits.push(!bits[n - 1]);

    // Build run-length array starting with a run of 1s (possibly length 0),
    // alternating #1, #0, #1, #0, ...
    let mut arr: Vec<i32> = Vec::new();
    let mut prev = true; // start expecting '1' run
    let mut cnt: i32 = 0;
    for &b in &bits {
        if b == prev {
            cnt += 1;
        } else {
            arr.push(cnt);
            prev = b;
            cnt = 1;
        }
    }

    // Prefix sums cum where cum[i] = sum(arr[0..i]).
    let mut cum: Vec<i32> = Vec::with_capacity(arr.len() + 1);
    cum.push(0);
    for &v in &arr {
        let last = *cum.last().unwrap();
        cum.push(last + v);
    }

    // Sliding over starting indices at 1-runs (even indices), taking up to 2K+1 runs.
    let ranges = 2 * k + 1;
    let mut ans: i32 = 0;

    // In the original C, cum is a large zero-initialized static array, so reading beyond
    // computed prefix values yields 0 and can produce incorrect results if not guarded.
    // The intended behavior per spec is to consider only valid concatenations of runs,
    // so we clamp the end index to arr.len().
    let aidx = arr.len();
    for i in (0..aidx).step_by(2) {
        let end = std::cmp::min(i + ranges, aidx);
        let val = cum[end] - cum[i];
        if val > ans {
            ans = val;
        }
        if i + ranges > aidx {
            break;
        }
    }

    println!("{}", ans);
}
