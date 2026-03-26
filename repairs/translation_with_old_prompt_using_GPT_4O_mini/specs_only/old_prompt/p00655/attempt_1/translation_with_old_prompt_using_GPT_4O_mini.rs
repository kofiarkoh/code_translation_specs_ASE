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
            self.second_half.pop_back();
        } else {
            self.first_half.pop_back();
        }
    }

    fn gmin(&self) -> Option<i32> {
        // Get minimum from the first half if not empty, otherwise from the second half
        if let Some(&min) = self.first_half.iter().min() {
            Some(min)
        } else {
            self.second_half.iter().min().copied()
        }
    }

    fn gmax(&self) -> Option<i32> {
        // Get maximum from the first half if not empty, otherwise from the second half
        if let Some(&max) = self.first_half.iter().max() {
            Some(max)
        } else {
            self.second_half.iter().max().copied()
        }
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
