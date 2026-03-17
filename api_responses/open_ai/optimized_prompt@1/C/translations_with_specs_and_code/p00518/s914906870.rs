use std::io::{self, BufRead};

const MOD: i32 = 10007;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the length of the sequence
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    // Read the sequence of characters
    let input: String = lines.next().unwrap().unwrap();

    // Preconditions
    assert!(n >= 0);
    assert_eq!(input.len(), n);
    assert!(input.chars().all(|c| c == 'J' || c == 'O' || c == 'I'));

    let mut key_prev = 1; // Initial key is 'J'
    let mut key_prev_prev = 0;
    let mut schedule = [0; 8];
    let mut schedule_next = [0; 8];

    schedule[1] = 1; // Starting with 'J'

    for (i, c) in input.chars().enumerate() {
        let key = match c {
            'J' => 1,
            'O' => 2,
            'I' => 4,
            _ => unreachable!(), // This should never happen due to precondition
        };

        schedule_next = [0; 8]; // Reset schedule_next

        if i == 0 {
            for j in 0..=7 {
                if (j & (key | key_prev)) == (key | key_prev) {
                    schedule_next[j] = 1;
                }
            }
            key_prev_prev = key_prev;
            key_prev = key;
            schedule.copy_from_slice(&schedule_next);
            continue;
        }

        for j in 0..=7 {
            if (j & (key | key_prev)) != 0 {
                for k in 0..=7 {
                    if (k & j) != 0 && (k & key) != 0 {
                        schedule_next[k] = (schedule_next[k] + schedule[j]) % MOD;
                    }
                }
            }
        }

        key_prev_prev = key_prev;
        key_prev = key;
        schedule.copy_from_slice(&schedule_next);
    }

    let conv: i32 = schedule.iter().sum::<i32>() % MOD;

    // Output the result
    println!("{}", conv);
}
