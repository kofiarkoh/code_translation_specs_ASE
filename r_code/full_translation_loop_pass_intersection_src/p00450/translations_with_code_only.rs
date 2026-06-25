use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Fixed-size array as in the C code: int go[100000];
    let mut go = vec![0i32; 100_000];

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // EOF: C would stop because scanf would fail; here we just exit.
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }
        let n_usize = n as usize;

        for i in 0..n_usize {
            go[i] = it.next().unwrap().parse::<i32>().unwrap();
        }

        // for (i = 1; i < n; i += 2) { ... }
        let mut i = 1usize;
        while i < n_usize {
            if go[i - 1] != go[i] {
                // for (j = i - 1; j >= 0 && go[j] != go[i]; j--) { go[j] = go[i]; }
                // In C, j is int and the loop condition checks j >= 0 before indexing.
                // In Rust with usize, we emulate the same by breaking when j would go below 0.
                let mut j = i - 1;
                while go[j] != go[i] {
                    go[j] = go[i];
                    if j == 0 {
                        break;
                    }
                    j -= 1;
                }
            }
            i += 2;
        }

        // for (i = cnt = 0; i < n; i++) cnt += !go[i];
        // In C, !x is 1 if x==0 else 0.
        let mut cnt: i32 = 0;
        for k in 0..n_usize {
            if go[k] == 0 {
                cnt += 1;
            }
        }

        print!("{cnt}\n");
    }
}
