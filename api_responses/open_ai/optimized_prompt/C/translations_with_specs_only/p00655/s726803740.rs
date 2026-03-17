use std::collections::VecDeque;
use std::io::{self, Write};

struct DualQueue {
    left: VecDeque<i32>,
    right: VecDeque<i32>,
}

impl DualQueue {
    fn new() -> Self {
        DualQueue {
            left: VecDeque::new(),
            right: VecDeque::new(),
        }
    }

    fn add(&mut self, value: i32) {
        // Add value to the left queue
        self.left.push_back(value);
        // Balance the queues if necessary
        self.balance();
    }

    fn delete(&mut self, from_left: bool) {
        if from_left {
            self.left.pop_front();
        } else {
            self.right.pop_front();
        }
        // Balance the queues after deletion
        self.balance();
    }

    fn min(&self, from_left: bool) -> Option<i32> {
        if from_left {
            self.left.front().copied() // Return the front of the left queue if it exists
        } else {
            self.right.front().copied() // Return the front of the right queue if it exists
        }
    }

    fn max(&self, from_left: bool) -> Option<i32> {
        if from_left {
            self.left.back().copied() // Return the back of the left queue if it exists
        } else {
            self.right.back().copied() // Return the back of the right queue if it exists
        }
    }

    fn balance(&mut self) {
        // Ensure both halves are balanced
        while self.left.len() > self.right.len() + 1 {
            if let Some(value) = self.left.pop_back() {
                self.right.push_front(value);
            }
        }
        while self.right.len() > self.left.len() {
            if let Some(value) = self.right.pop_back() {
                self.left.push_front(value);
            }
        }
    }
}

fn main() {
    let mut dual_queue = DualQueue::new();
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("Enter command: ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed

        if stdin.read_line(&mut input).is_err() {
            break; // Exit on read error
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue; // Skip empty input
        }

        match parts[0] {
            "add" if parts.len() == 2 => {
                if let Ok(value) = parts[1].parse::<i32>() {
                    dual_queue.add(value);
                }
            }
            "delete_left" => {
                dual_queue.delete(true);
            }
            "delete_right" => {
                dual_queue.delete(false);
            }
            "min_left" => {
                if let Some(min) = dual_queue.min(true) {
                    println!("Min left: {}", min);
                }
            }
            "min_right" => {
                if let Some(min) = dual_queue.min(false) {
                    println!("Min right: {}", min);
                }
            }
            "max_left" => {
                if let Some(max) = dual_queue.max(true) {
                    println!("Max left: {}", max);
                }
            }
            "max_right" => {
                if let Some(max) = dual_queue.max(false) {
                    println!("Max right: {}", max);
                }
            }
            "exit" => {
                println!("Terminating program.");
                break; // Exit the loop
            }
            _ => {
                println!("Invalid command.");
            }
        }
    }
}
