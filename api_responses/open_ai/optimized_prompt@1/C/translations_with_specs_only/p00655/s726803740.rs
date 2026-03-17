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
        // No error handling for invalid queue_number as per specification
    }

    fn delete(&mut self, queue_number: usize) -> Option<i32> {
        // Deletes an element from the specified queue and returns it
        if queue_number == 1 {
            if !self.queue1.is_empty() {
                return self.queue1.pop(); // Removes last element
            }
        } else if queue_number == 2 {
            if !self.queue2.is_empty() {
                return self.queue2.pop(); // Removes last element
            }
        }
        // If the queue is empty, return None (no error handling as per spec)
        None
    }

    fn min(&self, queue_number: usize) -> Option<i32> {
        // Retrieves the minimum value from the specified queue
        if queue_number == 1 {
            return self.queue1.iter().cloned().min(); // Returns minimum or None
        } else if queue_number == 2 {
            return self.queue2.iter().cloned().min(); // Returns minimum or None
        }
        None // No error handling for invalid queue_number
    }

    fn max(&self, queue_number: usize) -> Option<i32> {
        // Retrieves the maximum value from the specified queue
        if queue_number == 1 {
            return self.queue1.iter().cloned().max(); // Returns maximum or None
        } else if queue_number == 2 {
            return self.queue2.iter().cloned().max(); // Returns maximum or None
        }
        None // No error handling for invalid queue_number
    }
}

fn main() {
    let mut dual_queue = DualQueue::new();
    
    // Example usage (not specified in the task, but for demonstration)
    dual_queue.add(10, 1);
    dual_queue.add(20, 1);
    dual_queue.add(5, 2);
    
    if let Some(min) = dual_queue.min(1) {
        println!("Min of queue 1: {}", min);
    }
    
    if let Some(max) = dual_queue.max(2) {
        println!("Max of queue 2: {}", max);
    }
    
    if let Some(deleted) = dual_queue.delete(1) {
        println!("Deleted from queue 1: {}", deleted);
    }
    
    println!("End of processing.");
}
