use std::io::{self, BufRead, Read};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let n: usize = line.trim().parse().expect("Input must be a valid integer");

        if n == 0 {
            break; // Termination condition
        }

        let mut a = Vec::with_capacity(n);
        let mut inner_reader = reader.by_ref(); // Create a new reader for inner loop
        for _ in 0..n {
            let value: i32 = inner_reader
                .lines()
                .next()
                .expect("Failed to read value")
                .expect("Input must be a valid integer")
                .trim()
                .parse()
                .expect("Input must be a valid integer");
            a.push(value);
        }

        let swap_count = bubble_sort_and_count_swaps(&mut a);
        println!("{}", swap_count);
    }
}

fn bubble_sort_and_count_swaps(arr: &mut Vec<i32>) -> usize {
    let mut swap_count = 0;
    let n = arr.len();

    for u in (1..n).rev() {
        for i in 0..u {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1); // Safe swap
                swap_count += 1;
            }
        }
    }
    swap_count
}
