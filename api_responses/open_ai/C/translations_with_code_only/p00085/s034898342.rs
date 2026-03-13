use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers, similar to repeated scanf("%d %d", &n, &m)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let tokens: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut pos = 0;
    while pos + 1 < tokens.len() {
        let n = tokens[pos];
        let m = tokens[pos + 1];
        pos += 2;

        // The C loop condition is (scanf(...), m + n != 0)
        if n + m == 0 {
            break;
        }

        // In the original C code d is a local array of size 1000 and the code assumes n elements.
        // We create a Vec of length n and initialize elements to 1, matching the for-loop d[i] = 1.
        // Note: If n <= 0 the original C code exhibits undefined/implementation-defined behaviour
        // (accessing uninitialized/out-of-range elements). This Rust translation will panic on such inputs,
        // which is a reasonable fail-fast analogue.
        let n_usize = n as usize;
        let mut d = vec![1i32; n_usize];

        let mut i: usize = 0;
        let mut sum: i32 = 0;
        let mut f: i32 = 0;

        loop {
            // This mirrors: sum += d[i];
            sum += d[i];

            // if (sum == m) { d[i]=0; sum=0; f++; }
            if sum == m {
                d[i] = 0;
                sum = 0;
                f += 1;
            }

            // if (f == n) break;
            if f == n {
                break;
            }

            // if (i == n - 1) { i = 0; continue; } i++;
            if i == n_usize - 1 {
                i = 0;
                continue;
            }
            i += 1;
        }

        // printf("%d\n", i + 1);
        println!("{}", i + 1);
    }
}
