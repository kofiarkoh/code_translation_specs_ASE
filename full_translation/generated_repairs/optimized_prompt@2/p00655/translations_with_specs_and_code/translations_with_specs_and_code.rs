use std::io::{self, BufRead};

const FH: usize = 0; // First half
const SH: usize = 1; // Second half
const ALL: usize = 2;
const MAX: usize = 200005;

struct Queue {
    data: Vec<i32>,
    min: Vec<i32>,
    max: Vec<i32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            data: Vec::new(),
            min: Vec::new(),
            max: Vec::new(),
        }
    }

    fn add(&mut self, value: i32) {
        self.data.push(value);
        while let Some(&last) = self.min.last() {
            if last <= value {
                break;
            }
            self.min.pop();
        }
        self.min.push(value);
        while let Some(&last) = self.max.last() {
            if last >= value {
                break;
            }
            self.max.pop();
        }
        self.max.push(value);
    }

    fn del(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        let value = self.data.remove(0);
        if let Some(&min_val) = self.min.first() {
            if min_val == value {
                self.min.remove(0);
            }
        }
        if let Some(&max_val) = self.max.first() {
            if max_val == value {
                self.max.remove(0);
            }
        }
        Some(value)
    }

    fn gmin(&self, th: usize) -> Option<i32> {
        if th == 0 || th > self.min.len() {
            return None;
        }
        Some(self.min[th - 1])
    }

    fn gmax(&self, th: usize) -> Option<i32> {
        if th == 0 || th > self.max.len() {
            return None;
        }
        Some(self.max[th - 1])
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    while let Some(line) = lines.next() {
        let q: usize = line.trim().parse().expect("Failed to parse number of operations");
        let mut queues = [Queue::new(), Queue::new()];

        for _ in 0..q {
            let op: usize = lines.next().unwrap().trim().parse().expect("Failed to parse operation");
            let mut a: Option<i32> = None;

            match op {
                0 => {
                    let value: i32 = lines.next().unwrap().trim().parse().expect("Failed to parse value");
                    add(&mut queues, value);
                }
                1 => {
                    a = del(&mut queues);
                }
                2 => {
                    a = queues[FH].gmin(1);
                }
                3 => {
                    a = queues[SH].gmin(1);
                }
                4 => {
                    let th: usize = lines.next().unwrap().trim().parse().expect("Failed to parse threshold");
                    a = queues[FH].gmin(th);
                }
                5 => {
                    let th: usize = lines.next().unwrap().trim().parse().expect("Failed to parse threshold");
                    a = queues[SH].gmin(th);
                }
                6 => {
                    a = queues[FH].gmax(1);
                }
                7 => {
                    a = queues[SH].gmax(1);
                }
                8 => {
                    let th: usize = lines.next().unwrap().trim().parse().expect("Failed to parse threshold");
                    a = queues[FH].gmax(th);
                }
                _ => {
                    let th: usize = lines.next().unwrap().trim().parse().expect("Failed to parse threshold");
                    a = queues[SH].gmax(th);
                }
            }

            if let Some(value) = a {
                println!("{}", value);
            }
        }
        println!("end");
    }
}

fn add(queues: &mut [Queue; 2], value: i32) {
    queues[SH].add(value);
    if queues[FH].data.len() < queues[SH].data.len() {
        if let Some(val) = queues[SH].del() {
            queues[FH].add(val);
        }
    }
}

fn del(queues: &mut [Queue; 2]) -> Option<i32> {
    let mut value = queues[FH].del();
    if queues[FH].data.len() < queues[SH].data.len() {
        if let Some(val) = queues[SH].del() {
            queues[FH].add(val);
        }
    }
    value
}
