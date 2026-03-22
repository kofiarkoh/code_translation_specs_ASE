use std::io::{self, Read};

fn main() {
    // Read all input (no validation per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = (0..n)
        .map(|_| it.next().unwrap().parse::<i64>().unwrap())
        .collect();

    a.sort();

    // S = sum |a[i]|
    let s: i64 = a.iter().map(|&v| v.abs()).sum();

    // Collect output lines to print once.
    let mut out = String::new();

    if a[0] >= 0 {
        // All non-negative: print S - 2*a[0]
        let ans = s - 2 * a[0];
        out.push_str(&format!("{ans}\n"));

        // Operations
        let mut x = a[0];
        if n >= 3 {
            for i in 1..=n - 2 {
                out.push_str(&format!("{} {}\n", x, a[i]));
                x -= a[i];
            }
        }
        out.push_str(&format!("{} {}\n", a[n - 1], x));
    } else if a[n - 1] <= 0 {
        // All non-positive: print S + 2*a[n-1]
        let ans = s + 2 * a[n - 1];
        out.push_str(&format!("{ans}\n"));

        // Operations
        let mut x = a[n - 1];
        for i in 0..=n - 2 {
            out.push_str(&format!("{} {}\n", x, a[i]));
            x -= a[i];
        }
    } else {
        // Mixed signs: print S
        out.push_str(&format!("{s}\n"));

        // Find l: last index with a[l] < 0 and a[l+1] >= 0.
        // Use a binary-search-like loop; well-defined under mixed-sign condition.
        let mut lo: isize = 0;
        let mut hi: isize = (n as isize) - 1;
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if a[mid as usize] < 0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let l: usize = lo as usize;

        fn dfs(
            a: &[i64],
            f: usize,
            b: usize,
            l: usize,
            sign: i32,
            out: &mut String,
        ) -> i64 {
            // Base case 1: f == l
            if f == l {
                let mut sum = a[l];
                if sign == 1 {
                    // for i = l+1 .. b-1
                    if l + 1 <= b.saturating_sub(1) {
                        for i in (l + 1)..b {
                            out.push_str(&format!("{} {}\n", sum, a[i]));
                            sum -= a[i];
                        }
                    }
                    out.push_str(&format!("{} {}\n", a[b], sum));
                    sum = a[b] - sum;
                    return sum;
                } else {
                    // sign == -1: for i = l+1 .. b
                    for i in (l + 1)..=b {
                        out.push_str(&format!("{} {}\n", sum, a[i]));
                        sum -= a[i];
                    }
                    return sum;
                }
            }

            // Base case 2: b == l+1
            if b == l + 1 {
                let mut sum = a[l + 1];
                if sign == 1 {
                    // for i = f .. l
                    for i in f..=l {
                        out.push_str(&format!("{} {}\n", sum, a[i]));
                        sum -= a[i];
                    }
                    return sum;
                } else {
                    // sign == -1: for i = f .. l-1, then (a[l], sum)
                    if f <= l - 1 {
                        for i in f..=l - 1 {
                            out.push_str(&format!("{} {}\n", sum, a[i]));
                            sum -= a[i];
                        }
                    }
                    out.push_str(&format!("{} {}\n", a[l], sum));
                    sum = a[l] - sum;
                    return sum;
                }
            }

            // Recursive case
            if sign == 1 {
                out.push_str(&format!("{} {}\n", a[b], a[f]));
                let x = dfs(a, f + 1, b - 1, l, -sign, out);
                let t = a[b] - a[f];
                out.push_str(&format!("{} {}\n", t, x));
                t - x
            } else {
                out.push_str(&format!("{} {}\n", a[f], a[b]));
                let x = dfs(a, f + 1, b - 1, l, -sign, out);
                let t = a[f] - a[b];
                out.push_str(&format!("{} {}\n", t, x));
                t - x
            }
        }

        // The returned value is not printed; the spec states it equals S as the final reduction.
        let _final_val = dfs(&a, 0, n - 1, l, 1, &mut out);
    }

    print!("{out}");
}
