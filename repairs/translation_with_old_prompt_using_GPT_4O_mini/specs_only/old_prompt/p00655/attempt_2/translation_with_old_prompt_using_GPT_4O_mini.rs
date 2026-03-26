use std::collections::VecDeque;
use std::io::{self, BufRead};

struct FimoQueue {
    first_half: VecDeque<i32>,
    second_half: VecDeque<i32>,
}

impl FimoQueue {
    fn new() -> Self {
        FimoQueue {
            first_half: VecDeque::new(),
            second_half: VecDeque::new(),
        }
    }

    fn add(&mut self, value: i32) {
        // Add to the first half
        self.first_half.push_back(value);
    }

    fn delete(&mut self) {
        // Remove from the first half if not empty, otherwise from the second half
        if self.first_half.is_empty() {
            self.second_half.pop_front(); // Changed to pop_front to maintain FIFO
        } else {
            self.first_half.pop_front(); // Changed to pop_front to maintain FIFO
        }
    }

    fn gmin(&self) -> Option<i32> {
        // Get minimum from the first half if not empty, otherwise from the second half
        let min_first = self.first_half.iter().min().copied();
        let min_second = self.second_half.iter().min().copied();
        min_first.or(min_second) // Return the minimum from either half
    }

    fn gmax(&self) -> Option<i32> {
        // Get maximum from the first half if not empty, otherwise from the second half
        let max_first = self.first_half.iter().max().copied();
        let max_second = self.second_half.iter().max().copied();
        max_first.or(max_second) // Return the maximum from either half
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut fimo_queue = FimoQueue::new();

    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let op = parts[0];
        let a = parts.get(1).copied();

        match op {
            0 => {
                if let Some(value) = a {
                    fimo_queue.add(value);
                }
            }
            1 => {
                fimo_queue.delete();
            }
            2 => {
                if let Some(min) = fimo_queue.gmin() {
                    println!("{}", min);
                }
            }
            3 => {
                if let Some(max) = fimo_queue.gmax() {
                    println!("{}", max);
                }
            }
            _ => {}
        }
    }
    println!("end");
}
