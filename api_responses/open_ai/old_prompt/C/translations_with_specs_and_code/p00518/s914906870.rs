use std::io::{self, BufRead};

const MOD: i32 = 10007;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the length of the input string
    let mut input_line = String::new();
    handle.read_line(&mut input_line).unwrap();
    let n: usize = input_line.trim().parse().unwrap();

    // Read the input string
    let mut input_string = String::new();
    handle.read_line(&mut input_string).unwrap();
    let input_string = input_string.trim();

    // Ensure the input length matches N
    assert_eq!(input_string.len(), n);

    let mut key_prev = 1; // Initial key is 'J'
    let mut key_prev_prev = 0;
    let mut schedule = [0; 8];
    let mut schedule_next = [0; 8];

    schedule[1] = 1; // Start with 'J'

    for i in 0..n {
        let key = match input_string.chars().nth(i).unwrap() {
            'J' => 1,
            'O' => 2,
            'I' => 4,
            _ => unreachable!(), // Input is guaranteed to be valid
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
