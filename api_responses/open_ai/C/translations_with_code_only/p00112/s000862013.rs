use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut time = vec![0i64; 10001];
    let mut wtime = vec![0i64; 10001];

    loop {
        let mut n_str = String::new();
        input.read_line(&mut n_str).unwrap();
        let n: usize = n_str.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        for j in 0..n {
            let mut time_str = String::new();
            input.read_line(&mut time_str).unwrap();
            time[j] = time_str.trim().parse().unwrap();
        }

        for j in 0..n - 1 {
            let mut min = j;
            for k in j + 1..n {
                if time[min] > time[k] {
                    min = k;
                }
            }
            if min != j {
                // Swap time[j] and time[min] using arithmetic to preserve semantics
                time[min] += time[j];
                time[j] = time[min] - time[j];
                time[min] -= time[j];
            }
        }

        let mut sum = 0;
        for j in 1..n {
            wtime[j] = wtime[j - 1] + time[j - 1];
            sum += wtime[j];
        }

        println!("{}", sum);
    }
}
