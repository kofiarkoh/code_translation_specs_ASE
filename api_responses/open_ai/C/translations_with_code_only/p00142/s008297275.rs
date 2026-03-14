use std::io::{self, BufRead};

fn main() {
    let mut exp = [0; 10001];
    let mut mod_arr = [0; 10000]; // mod[1] to mod[n-1]
    let mut a = [0; 10000]; // a[0] to a[c-1]
    let mut count = [0; 6000]; // count[1] to count[h]
    
    // Precompute squares
    for i in 1..10001 {
        exp[i] = i * i;
    }

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let n: i32 = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        // Reset count array
        count.fill(0);
        
        let mut c = 0;

        // Compute mod values
        for i in 1..n {
            mod_arr[i as usize] = exp[i as usize] % n;
        }

        // Collect unique mod values
        for i in 1..n {
            let mut j = 0;
            while j < c {
                if mod_arr[i as usize] == a[j as usize] {
                    break;
                }
                j += 1;
            }
            if j == c {
                a[c as usize] = mod_arr[i as usize];
                c += 1;
            }
        }

        let h = (n - 1) / 2;

        // Count differences
        for i in 0..c {
            for j in 0..c {
                if a[i as usize] != a[j as usize] {
                    let mut d = a[i as usize] - a[j as usize];
                    if d < 0 {
                        d += n;
                    }
                    if d > h {
                        d = n - d;
                    }
                    count[d as usize] += 1;
                }
            }
        }

        // Output results
        for i in 1..=h {
            println!("{}", count[i as usize]);
        }
    }
}
