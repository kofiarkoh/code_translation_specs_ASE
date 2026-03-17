use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = reader.lines().map(|line| line.unwrap());

    loop {
        let n: i32 = input.next().unwrap().parse().unwrap();
        let p: i32 = input.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut i = 0;
        let mut w = p;
        let mut c = vec![0; 50]; // Initialize the counter array

        loop {
            if w > 0 {
                w -= 1;
                c[i as usize % n as usize] += 1;

                if w == 0 {
                    let mut found = false;
                    for j in 0..n {
                        if j != i % n {
                            if c[j as usize] > 0 {
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        println!("{}", i % n);
                        break;
                    }
                }
            } else {
                w += c[i as usize % n as usize];
                c[i as usize % n as usize] = 0;
            }
            i += 1;
        }
    }
}
