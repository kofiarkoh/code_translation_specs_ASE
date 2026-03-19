use std::io::{self, Read};

const MAX: usize = 13;

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }
        let n_usize = n as usize;

        // C arrays: int numline[MAX] = {}; int tmpline[MAX] = {}; int num[101] = {};
        // Keep fixed-size arrays to mirror bounds/behavior; n is assumed <= MAX by the C code.
        let mut numline = [0i32; MAX];
        let mut tmpline = [0i32; MAX];
        let mut num = [0i32; 101];

        for i in 0..n_usize {
            // If input is malformed/short, C would have undefined behavior; we unwrap like typical contest code.
            numline[i] = it.next().unwrap().parse().unwrap();
        }

        let mut flag: i32 = 1;
        let mut cnt: i32 = 0;

        while flag != 0 {
            // for (i=0; i<101; i++) num[i]=0;
            num.fill(0);

            // for (i=0; i<n; i++) num[numline[i]]++;
            // In C, out-of-range indices are UB; we assume inputs keep indices within 0..=100.
            for i in 0..n_usize {
                let idx = numline[i] as usize;
                num[idx] += 1;
            }

            // for (i=0; i<n; i++) tmpline[i] = num[numline[i]];
            for i in 0..n_usize {
                let idx = numline[i] as usize;
                tmpline[i] = num[idx];
            }

            cnt += 1;
            flag = 0;

            // Count positions where tmpline[i] == numline[i]
            for i in 0..n_usize {
                if tmpline[i] == numline[i] {
                    flag += 1;
                }
            }

            if flag == n {
                flag = 0;
            } else {
                // Copy tmpline -> numline and continue
                for i in 0..n_usize {
                    numline[i] = tmpline[i];
                }
                flag = 1;
            }
        }

        // Output exactly as C: cnt-1, then tmpline values separated by spaces, newline at end.
        print!("{}\n", cnt - 1);
        for i in 0..n_usize {
            if i + 1 == n_usize {
                print!("{}\n", tmpline[i]);
            } else {
                print!("{} ", tmpline[i]);
            }
        }
    }
}
