use std::collections::VecDeque;
use std::io::{self, Write};

struct DualQueue {
    queue1: VecDeque<i32>,
    queue2: VecDeque<i32>,
}

impl DualQueue {
    fn new() -> Self {
        DualQueue {
            queue1: VecDeque::new(),
            queue2: VecDeque::new(),
        }
    }

    fn add(&mut self, value: i32, queue_number: usize) {
        // Add value to the specified queue (1 or 2)
        match queue_number {
            1 => self.queue1.push_back(value),
            2 => self.queue2.push_back(value),
            _ => {} // Invalid queue number, do nothing
        }
    }

    fn delete(&mut self, queue_number: usize) -> Option<i32> {
        // Remove and return the front element of the specified queue (1 or 2)
        match queue_number {
            1 => self.queue1.pop_front(),
            2 => self.queue2.pop_front(),
            _ => None, // Invalid queue number, return None
        }
    }

    fn min(&self, queue_number: usize) -> Option<i32> {
        // Return the minimum value from the specified queue (1 or 2)
        match queue_number {
            1 => self.queue1.iter().cloned().min(),
            2 => self.queue2.iter().cloned().min(),
            _ => None, // Invalid queue number, return None
        }
    }

    fn max(&self, queue_number: usize) -> Option<i32> {
        // Return the maximum value from the specified queue (1 or 2)
        match queue_number {
            1 => self.queue1.iter().cloned().max(),
            2 => self.queue2.iter().cloned().max(),
            _ => None, // Invalid queue number, return None
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
            "add" if parts.len() == 3 => {
                if let Ok(value) = parts[1].parse::<i32>() {
                    if let Ok(queue_number) = parts[2].parse::<usize>() {
                        dual_queue.add(value, queue_number);
                    }
                }
            }
            "delete" if parts.len() == 2 => {
                if let Ok(queue_number) = parts[1].parse::<usize>() {
                    if let Some(value) = dual_queue.delete(queue_number) {
                        println!("Deleted: {}", value);
                    } else {
                        println!("Delete operation failed: Queue {} is empty.", queue_number);
                    }
                }
            }
            "min" if parts.len() == 2 => {
                if let Ok(queue_number) = parts[1].parse::<usize>() {
                    if let Some(value) = dual_queue.min(queue_number) {
                        println!("Min: {}", value);
                    } else {
                        println!("Min operation failed: Queue {} is empty.", queue_number);
                    }
                }
            }
            "max" if parts.len() == 2 => {
                if let Ok(queue_number) = parts[1].parse::<usize>() {
                    if let Some(value) = dual_queue.max(queue_number) {
                        println!("Max: {}", value);
                    } else {
                        println!("Max operation failed: Queue {} is empty.", queue_number);
                    }
                }
            }
            "exit" => break, // Exit command
            _ => println!("Invalid command."), // Notify invalid commands
        }
    }

    println!("End of processing.");
}
