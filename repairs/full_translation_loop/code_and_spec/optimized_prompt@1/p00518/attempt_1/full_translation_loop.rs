use std::io::{self, BufRead};

const MOD: i32 = 10007;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the length of the sequence
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Invalid input for N");

    // Read the sequence of characters
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let in_seq = input.trim();

    // Preconditions
    assert!(n >= 0);
    assert_eq!(in_seq.len(), n);
    assert!(in_seq.chars().all(|c| c == 'J' || c == 'O' || c == 'I'));

    let mut key_prev = 1; // Initial key is 'J'
    let mut key_prev_prev = 0;
    let mut schedule = [0; 8];
    let mut schedule_next = [0; 8];

    schedule[1] = 1; // Start with 'J'

    for (i, c) in in_seq.chars().enumerate() {
        let key = match c {
            'J' => 1,
            'O' => 2,
            'I' => 4,
            _ => unreachable!(), // This should never happen due to precondition
        };

        schedule_next = [0; 8]; // Reset the next schedule

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
    println!("{}", conv);
}
