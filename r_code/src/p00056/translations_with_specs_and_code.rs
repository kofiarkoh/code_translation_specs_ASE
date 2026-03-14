fn main() {
    let mut prime = vec![false; 5001];
    set(&mut prime);

    let stdin = std::io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let num: i32 = match input.trim().parse() {
            Ok(n) if n >= 0 => n,
            _ => break, // Terminate on invalid input or negative number
        };

        if num == 0 {
            break; // Terminate on zero input
        }

        let count = count_prime_pairs(num, &prime);
        println!("{}", count);
    }
}

fn set(prime: &mut Vec<bool>) {
    for i in (4..=5000).step_by(2) {
        prime[i] = true; // Mark even numbers as non-prime
    }
    for i in (3..=5000).step_by(2) {
        if !prime[i] {
            for o in (i * 2..=5000).step_by(i) {
                prime[o] = true; // Mark multiples of prime as non-prime
            }
        }
    }
}

fn count_prime_pairs(num: i32, prime: &[bool]) -> i32 {
    let mut count = 0;

    for i in 2..=num {
        if !prime[i as usize] {
            for o in 2..=num {
                if !prime[o as usize] {
                    let sum = i + o;
                    if sum == num {
                        count += 1;
                        break; // Found a valid pair, break to avoid counting duplicates
                    }
                    if sum > num {
                        break; // No need to check further if sum exceeds num
                    }
                }
            }
        }
    }

    // Adjust count to account for distinct pairs
    count / 2 + count % 2
}
