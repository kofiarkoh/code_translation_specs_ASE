struct DynamicArray {
    s: Vec<i32>, // The dynamic array to hold sorted integers
    cnt: usize,  // Current count of elements in the array
}

impl DynamicArray {
    // Create a new DynamicArray
    fn new() -> Self {
        DynamicArray {
            s: Vec::new(),
            cnt: 0,
        }
    }

    // Insert a new element into the array
    fn insert(&mut self, key: i32) -> i32 {
        // Find the position to insert the new key
        let pos = self.get_lower_bound(key);
        if pos < self.cnt && self.s[pos] == key {
            return 0; // Element already exists
        }
        // Insert the element at the found position
        self.s.insert(pos, key);
        self.cnt += 1; // Increment the count
        1 // Successful insertion
    }

    // Delete an element from the array
    fn delete(&mut self, key: i32) -> i32 {
        let pos = self.search(key);
        if pos == -1 {
            return -1; // Element not found
        }
        // Remove the element at the found position
        self.s.remove(pos as usize); // Convert pos to usize
        self.cnt -= 1; // Decrement the count
        1 // Successful deletion
    }

    // Search for an element in the array
    fn search(&self, key: i32) -> i32 {
        let mut low = 0;
        let mut high = self.cnt as isize - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if self.s[mid as usize] == key {
                return mid as i32; // Element found
            } else if self.s[mid as usize] < key {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        -1 // Element not found
    }

    // Get the lower bound for insertion
    fn get_lower_bound(&self, key: i32) -> usize {
        let mut low = 0;
        let mut high = self.cnt as isize;

        while low < high {
            let mid = (low + high) / 2;
            if self.s[mid as usize] < key {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low as usize // Return the position for insertion
    }

    // Dump elements within a specified range
    fn dump(&self, head: usize, tail: usize) {
        if head > tail || head >= self.cnt {
            return; // Invalid range
        }
        for &value in &self.s[head..=tail.min(self.cnt - 1)] {
            println!("{}", value); // Print each element in the range
        }
    }

    // Print the current count and elements of the array
    fn print(&self) {
        println!("{}", self.cnt); // Print the count directly
        for &value in &self.s {
            println!("{}", value); // Print each element
        }
    }
}

// Main function to demonstrate usage
fn main() {
    let mut array = DynamicArray::new();
    
    // Sample input processing
    let inputs = vec![
        (0, 5), (0, 3), (0, 8), (1, 5), (1, 10), 
        (2, 3), (1, 3), (3, 3), (0, 7), (1, 7)
    ];
    
    for (op, key) in inputs {
        match op {
            0 => println!("{}", array.insert(key)), // Insert operation
            1 => println!("{}", array.delete(key)), // Delete operation
            2 => println!("{}", array.search(key)), // Search operation
            3 => array.dump(key as usize, key as usize), // Dump operation
            _ => {}
        }
    }
}
