use std::io::{self, BufRead};

const MOD: i32 = 10007;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let in_str: String = lines.next().unwrap().unwrap();

    let mut key_prev = 1; // 最初に鍵を持っているのはJ
    let mut key_prev_prev = 0;

    let mut schedule = [0; 8];
    let mut schedule_next = [0; 8];
    schedule[1] = 1;

    for i in 0..n {
        let key = match in_str.chars().nth(i).unwrap() {
            'J' => 1,
            'O' => 2,
            'I' => 4,
            _ => continue, // Handle unexpected characters gracefully
        };

        schedule_next = [0; 8];

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
                        schedule_next[k] += schedule[j];
                        schedule_next[k] %= MOD;
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
