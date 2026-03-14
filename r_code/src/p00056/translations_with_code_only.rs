use std::io::{self, BufRead};

const MAX: usize = 5000;

fn set(prime: &mut [u8; 50001]) {
    for i in (4..=MAX).step_by(2) {
        prime[i] = 1; // Mark even numbers as non-prime
    }
    for i in (3..=MAX).step_by(2) {
        if prime[i] == 0 {
            for o in (i * 2..=MAX).step_by(i) {
                prime[o] = 1; // Mark multiples of prime as non-prime
            }
        }
    }
}

fn main() {
    let mut prime = [0; 50001];
    set(&mut prime);

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut count: i32;

    for line in handle.lines() {
        let num: i32 = line.unwrap().trim().parse().unwrap();
        if num == 0 {
            break; // Exit on 0 input
        }

        count = 0;
        for i in 2..=num {
            if prime[i as usize] == 0 {
                if i > num {
                    break; // No need to check further if i exceeds num
                }
                for o in 2..=num {
                    if prime[o as usize] == 0 {
                        let sum = i + o;
                        if sum == num {
                            count += 1; // Found a valid pair
                            break; // Move to the next i
                        }
                        if sum > num {
                            break; // No need to check further if sum exceeds num
                        }
                    }
                }
            }
        }
        // Output the result, adjusting count as per the original logic
        println!("{}", if count % 2 == 1 { count / 2 + 1 } else { count / 2 });
    }
}
