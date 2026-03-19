use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: usize = line.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut numline = vec![0; n];
        for i in 0..n {
            let value: usize = reader.lines().next().unwrap().unwrap().trim().parse().unwrap();
            numline[i] = value;
        }

        let mut cnt = 0;
        let mut stable = false;

        while !stable {
            let mut num = vec![0; 101];
            let mut tmpline = vec![0; n];

            // Count occurrences
            for &value in &numline {
                num[value] += 1;
            }

            // Transform the sequence
            for i in 0..n {
                tmpline[i] = num[numline[i]];
            }

            cnt += 1;
            stable = true;

            // Check if the sequence has stabilized
            for i in 0..n {
                if tmpline[i] != numline[i] {
                    stable = false;
                    break;
                }
            }

            // Update numline for the next iteration if not stable
            if !stable {
                numline.copy_from_slice(&tmpline);
            }
        }

        // Output the number of transformations and the final sequence
        println!("{}", cnt - 1);
        for (i, &value) in numline.iter().enumerate() {
            if i == n - 1 {
                println!("{}", value);
            } else {
                print!("{} ", value);
            }
        }
    }
}
