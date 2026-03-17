struct DualQueue {
    queue1: Vec<i32>, // First queue
    queue2: Vec<i32>, // Second queue
}

impl DualQueue {
    fn new() -> Self {
        DualQueue {
            queue1: Vec::new(),
            queue2: Vec::new(),
        }
    }

    fn add(&mut self, value: i32, queue_number: usize) {
        // Adds an element to the specified queue
        if queue_number == 1 {
            self.queue1.push(value);
        } else if queue_number == 2 {
            self.queue2.push(value);
        }
    }

    fn delete(&mut self, queue_number: usize) -> Option<i32> {
        // Deletes an element from the specified queue and returns it
        if queue_number == 1 {
            if !self.queue1.is_empty() {
                return self.queue1.remove(0); // Removes first element
            }
        } else if queue_number == 2 {
            if !self.queue2.is_empty() {
                return self.queue2.remove(0); // Removes first element
            }
        }
        None
    }

    fn min(&self, queue_number: usize) -> Option<i32> {
        // Retrieves the minimum value from the specified queue
        if queue_number == 1 {
            return self.queue1.iter().cloned().min(); // Returns minimum or None
        } else if queue_number == 2 {
            return self.queue2.iter().cloned().min(); // Returns minimum or None
        }
        None
    }

    fn max(&self, queue_number: usize) -> Option<i32> {
        // Retrieves the maximum value from the specified queue
        if queue_number == 1 {
            return self.queue1.iter().cloned().max(); // Returns maximum or None
        } else if queue_number == 2 {
            return self.queue2.iter().cloned().max(); // Returns maximum or None
        }
        None
    }
}

fn main() {
    let mut dual_queue = DualQueue::new();
    
    // Read input values
    let inputs = vec![
        (0, 5),
        (0, 3),
        (0, 8),
        (0, 1),
        (0, 6),
        (4, 2),
        (5, 1),
        (6, 1),
        (7, 1),
        (1, 0),
    ];

    for (queue_number, value) in inputs {
        if value == 0 {
            if let Some(min) = dual_queue.min(queue_number) {
                println!("{}", min);
            } else {
                println!("end");
            }
        } else {
            dual_queue.add(value, queue_number);
        }
    }

    // Process deletions
    for _ in 0..2 {
        if let Some(deleted) = dual_queue.delete(1) {
            println!("Deleted from queue 1: {}", deleted);
        }
    }

    // Print max of queue 2
    if let Some(max) = dual_queue.max(2) {
        println!("Max of queue 2: {}", max);
    } else {
        println!("end");
    }

    println!("End of processing.");
}
