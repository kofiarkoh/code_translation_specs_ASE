use std::io::{self, BufRead};

const MAX: usize = 13;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: usize = line.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut numline = vec![0; MAX];
        let mut tmpline = vec![0; MAX];
        let mut num = vec![0; 101];

        // Read the sequence of integers
        for i in 0..n {
            let value: usize = reader.lines().next().unwrap().unwrap().trim().parse().unwrap();
            numline[i] = value;
        }

        let mut cnt = 0;
        let mut flag = true;

        while flag {
            // Reset frequency counts
            for i in 0..101 {
                num[i] = 0;
            }

            // Count occurrences
            for &value in &numline[..n] {
                num[value] += 1;
            }

            // Transform the sequence
            for i in 0..n {
                tmpline[i] = num[numline[i]];
            }

            cnt += 1;
            flag = false;

            // Check if the sequence has stabilized
            for i in 0..n {
                if tmpline[i] == numline[i] {
                    flag = true;
                }
            }

            if !flag {
                // If not stabilized, update numline for the next iteration
                for i in 0..n {
                    numline[i] = tmpline[i];
                }
                flag = true;
            }
        }

        // Output the number of iterations and the final transformed sequence
        println!("{}", cnt - 1);
        for i in 0..n {
            if i == n - 1 {
                println!("{}", tmpline[i]);
            } else {
                print!("{} ", tmpline[i]);
            }
        }
    }
}
