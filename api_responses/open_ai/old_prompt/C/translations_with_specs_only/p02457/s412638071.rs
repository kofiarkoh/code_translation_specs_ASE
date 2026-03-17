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

    // Insert a new element into the array while maintaining sorted order
    fn insert(&mut self, key: i32) -> i32 {
        // Find the position to insert the new element
        let pos = self.get_lb(key);
        if pos < 0 {
            return 0; // Invalid position
        }
        // Insert the element at the found position
        self.s.insert(pos as usize, key);
        self.cnt += 1; // Increment the count
        1 // Successful insertion
    }

    // Delete an element from the array
    fn delete(&mut self, key: i32) -> i32 {
        // Find the index of the element to delete
        let index = self.search(key);
        if index < 0 {
            return -1; // Element not found
        }
        // Remove the element at the found index
        self.s.remove(index as usize);
        self.cnt -= 1; // Decrement the count
        1 // Successful deletion
    }

    // Search for an element and return its index or -1 if not found
    fn search(&self, key: i32) -> i32 {
        let mut low = 0;
        let mut high = self.cnt as isize - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if self.s[mid as usize] == key {
                return mid; // Element found
            } else if self.s[mid as usize] < key {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        -1 // Element not found
    }

    // Get the lower bound index for insertion
    fn get_lb(&self, key: i32) -> i32 {
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
        low // Return the lower bound index
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
        println!("Count: {}", self.cnt);
        for &value in &self.s {
            println!("{}", value); // Print each element
        }
    }
}
