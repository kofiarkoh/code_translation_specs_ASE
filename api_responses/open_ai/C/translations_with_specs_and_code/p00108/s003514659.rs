use std::io::{self, BufRead};

#[derive(Debug)]
struct Count {
    n: i32,
    count: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut counts: Vec<Count> = Vec::new();
    let mut seq = vec![0; 13];
    
    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = match iter.next() {
            Some(num) => num.parse().unwrap(),
            None => continue,
        };

        if n == 0 {
            break;
        }

        for i in 0..n {
            seq[i] = iter.next().unwrap().parse().unwrap();
        }

        let mut iterations = 0;
        loop {
            let mut fix = true;
            counts.clear();

            // Count occurrences
            for &num in &seq[..n] {
                let mut found = false;
                for count in &mut counts {
                    if count.n == num {
                        count.count += 1;
                        found = true;
                        break;
                    }
                }
                if !found {
                    counts.push(Count { n: num, count: 1 });
                }
            }

            // Transform sequence based on counts
            for j in 0..n {
                for count in &counts {
                    if count.n == seq[j] {
                        if seq[j] != count.count {
                            fix = false;
                        }
                        seq[j] = count.count;
                        break;
                    }
                }
            }

            iterations += 1;

            if fix {
                break;
            }
        }

        // Output the number of iterations
        println!("{}", iterations);
        // Output the final transformed sequence
        for i in 0..n {
            if i > 0 {
                print!(" ");
            }
            print!("{}", seq[i]);
        }
        println!();
    }
}
