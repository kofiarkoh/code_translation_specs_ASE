use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    while let Some(line) = input.next() {
        let line = line.unwrap();
        let mut params = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let D = params.next().unwrap();
        let N = params.next().unwrap();

        // Termination condition
        if D == 0 && N == 0 {
            break;
        }

        let mut a = vec![0; N];
        let mut e = vec![0; N];
        let mut r = vec![0; N];

        for i in 0..N {
            let line = input.next().unwrap().unwrap();
            let mut values = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
            a[i] = values.next().unwrap();
            e[i] = values.next().unwrap();
            r[i] = values.next().unwrap();
        }

        let result = min_items(D, N, &a, &e, &r);
        match result {
            Some(count) => println!("{}", count),
            None => println!("NA"),
        }
    }
}

fn min_items(D: usize, N: usize, a: &[usize], e: &[usize], r: &[usize]) -> Option<usize> {
    // Initialize memoization array
    let mut memo = vec![None; D + 1];
    memo[0] = Some(0); // Base case: 0 items needed to reach 0

    for i in 0..N {
        for j in (0..=D).rev() {
            if let Some(count) = memo[j] {
                let new_value = j + a[i];
                if new_value <= D {
                    let new_count = count + 1;
                    memo[new_value] = match memo[new_value] {
                        Some(existing_count) => Some(existing_count.min(new_count)),
                        None => Some(new_count),
                    };
                }
            }
        }
    }

    // Find the minimum number of items needed to reach or exceed D
    let mut min_count = None;
    for j in D..=D {
        if let Some(count) = memo[j] {
            min_count = match min_count {
                Some(existing_count) => Some(existing_count.min(count)),
                None => Some(count),
            };
        }
    }

    min_count
}
