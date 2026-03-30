use std::collections::VecDeque;

struct DualQueue {
    first_half: VecDeque<i32>,
    second_half: VecDeque<i32>,
}

impl DualQueue {
    fn new() -> Self {
        DualQueue {
            first_half: VecDeque::new(),
            second_half: VecDeque::new(),
        }
    }

    fn add(&mut self, value: i32) {
        // Add to the first half if it has fewer elements than the second half
        if self.first_half.len() < self.second_half.len() {
            self.first_half.push_back(value);
        } else {
            self.second_half.push_back(value);
        }
        self.balance();
    }

    fn delete(&mut self) {
        // Remove from the first half if it has more or equal elements than the second half
        if self.first_half.len() > 0 {
            self.first_half.pop_back();
        } else if self.second_half.len() > 0 {
            self.second_half.pop_back();
        }
        self.balance();
    }

    fn get_min_first(&self) -> Option<i32> {
        self.first_half.iter().cloned().min()
    }

    fn get_min_second(&self) -> Option<i32> {
        self.second_half.iter().cloned().min()
    }

    fn get_min_first_index(&self, index: usize) -> Option<i32> {
        self.first_half.get(index).cloned()
    }

    fn get_min_second_index(&self, index: usize) -> Option<i32> {
        self.second_half.get(index).cloned()
    }

    fn get_max_first(&self) -> Option<i32> {
        self.first_half.iter().cloned().max()
    }

    fn get_max_second(&self) -> Option<i32> {
        self.second_half.iter().cloned().max()
    }

    fn get_max_first_index(&self, index: usize) -> Option<i32> {
        self.first_half.get(index).cloned()
    }

    fn get_max_second_index(&self, index: usize) -> Option<i32> {
        self.second_half.get(index).cloned()
    }

    fn balance(&mut self) {
        // Ensure the first half is never smaller than the second half
        while self.first_half.len() < self.second_half.len() {
            if let Some(value) = self.second_half.pop_front() {
                self.first_half.push_back(value);
            }
        }
    }
}

fn main() {
    let mut dq = DualQueue::new();
    let operations = vec![
        (0, 5), // Add 5
        (0, 3), // Add 3
        (0, 8), // Add 8
        (0, 1), // Add 1
        (0, 6), // Add 6
        (4, 2), // Get min from first half at index 2
        (5, 1), // Get min from second half at index 1
        (6, 1), // Get max from first half at index 1
        (7, 1), // Get max from second half at index 1
        (1, 0), // Delete from first half
        (0, 0), // Add 0
    ];

    for (op, value) in operations {
        match op {
            0 => dq.add(value),
            1 => dq.delete(),
            2 => println!("{}", dq.get_min_first().unwrap_or_default()),
            3 => println!("{}", dq.get_min_second().unwrap_or_default()),
            4 => println!("{}", dq.get_min_first_index(value as usize).unwrap_or_default()),
            5 => println!("{}", dq.get_min_second_index(value as usize).unwrap_or_default()),
            6 => println!("{}", dq.get_max_first().unwrap_or_default()),
            7 => println!("{}", dq.get_max_second().unwrap_or_default()),
            8 => println!("{}", dq.get_max_first_index(value as usize).unwrap_or_default()),
            9 => println!("{}", dq.get_max_second_index(value as usize).unwrap_or_default()),
            _ => {}
        }
    }
    println!("end");
}
