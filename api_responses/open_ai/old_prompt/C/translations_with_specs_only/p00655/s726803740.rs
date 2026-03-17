use std::collections::VecDeque;
use std::io::{self, BufRead};

struct FimoSequence {
    first_half: VecDeque<i32>,
    second_half: VecDeque<i32>,
    min_first: Vec<i32>,
    max_first: Vec<i32>,
    min_second: Vec<i32>,
    max_second: Vec<i32>,
}

impl FimoSequence {
    fn new() -> Self {
        FimoSequence {
            first_half: VecDeque::new(),
            second_half: VecDeque::new(),
            min_first: Vec::new(),
            max_first: Vec::new(),
            min_second: Vec::new(),
            max_second: Vec::new(),
        }
    }

    fn add(&mut self, a: i32) {
        self.first_half.push_back(a);
        self.update_min_max_first();
    }

    fn delete(&mut self) {
        if !self.first_half.is_empty() {
            self.first_half.pop_front();
            self.update_min_max_first();
        }
    }

    fn gmin(&self) -> Option<i32> {
        if let Some(&min) = self.min_first.last() {
            return Some(min);
        }
        None
    }

    fn gmax(&self) -> Option<i32> {
        if let Some(&max) = self.max_first.last() {
            return Some(max);
        }
        None
    }

    fn update_min_max_first(&mut self) {
        if let Some(&first) = self.first_half.front() {
            self.min_first.push(first);
            self.max_first.push(first);
            for &value in self.first_half.iter().skip(1) {
                if value < *self.min_first.last().unwrap() {
                    self.min_first.push(value);
                }
                if value > *self.max_first.last().unwrap() {
                    self.max_first.push(value);
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut fimo = FimoSequence::new();

    for _ in 0..q {
        let input: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        let op = input[0];
        let a = input.get(1).copied().unwrap_or(0); // Default to 0 if not provided

        match op {
            0 => fimo.add(a),
            1 => fimo.delete(),
            2 => {
                if let Some(min) = fimo.gmin() {
                    println!("{}", min);
                }
            }
            3 => {
                if let Some(max) = fimo.gmax() {
                    println!("{}", max);
                }
            }
            _ => {}
        }
    }
    println!("end");
}
