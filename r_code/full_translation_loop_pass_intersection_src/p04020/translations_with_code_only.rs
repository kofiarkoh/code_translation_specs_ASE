use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // C uses 1-based indexing with A[100001]
    let mut a = vec![0i64; 100001];
    for i in 1..=n {
        if let Some(x) = it.next() {
            a[i] = x.parse::<i64>().unwrap();
        }
    }

    let mut ans: i64 = 0;
    for i in 1..n {
        ans = ans.wrapping_add(a[i] / 2);
        if a[i + 1] > 1 && (a[i] % 2 == 1) {
            a[i + 1] = a[i + 1].wrapping_sub(1);
            ans = ans.wrapping_add(1);
        }
    }
    ans = ans.wrapping_add(a[n] / 2);

    print!("{}\n", ans);
    // stdout is flushed on program exit; explicit flush not required to preserve output.
}
