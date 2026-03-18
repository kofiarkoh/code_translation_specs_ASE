use std::io::{self, Read};

fn enumeration(result: &mut [i32], used: &[i32], n: usize, d: usize, out: &mut String) {
    if d >= n {
        for i in 0..n {
            if i == 0 {
                out.push_str(&format!("{}", result[i]));
            } else {
                out.push_str(&format!(" {}", result[i]));
            }
        }
        out.push('\n');
        return;
    }

    // At depth d, `used` contains exactly the remaining (n - d) numbers.
    for i in 0..(n - d) {
        result[d] = used[i];

        // Build the remaining list excluding index i (mirrors C's VLA used2[n-d-1]).
        let mut used2 = Vec::with_capacity(n - d - 1);
        for (j, &v) in used.iter().enumerate() {
            if j != i {
                used2.push(v);
            }
        }

        enumeration(result, &used2, n, d + 1, out);
    }
}

fn main() {
    // Read entire stdin; behavior is undefined for invalid/negative input per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n_i32: i32 = input.split_whitespace().next().unwrap().parse().unwrap();
    let n: usize = n_i32 as usize;

    let used: Vec<i32> = (1..=n as i32).collect();
    let mut result: Vec<i32> = vec![0; n];

    let mut out = String::new();
    enumeration(&mut result, &used, n, 0, &mut out);
    print!("{out}");
}
