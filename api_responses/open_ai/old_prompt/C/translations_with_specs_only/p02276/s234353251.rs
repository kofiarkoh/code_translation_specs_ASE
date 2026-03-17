// Define the maximum size of the array
const N: usize = 1_000_000;

// Global array A
static mut A: [i32; N] = [0; N]; // Initialized with zeros

// Function to partition the array A
fn partition(p: usize, r: usize) -> usize {
    // Preconditions: p and r must be valid indices
    // Unsafe block is used to modify the global array A
    unsafe {
        // The pivot is the last element in the segment
        let pivot = A[r];
        let mut i = p as isize - 1; // i starts before the first index

        // Rearranging elements based on the pivot
        for j in p..r {
            if A[j] <= pivot {
                i += 1; // Move the boundary for elements <= pivot
                A.swap(i as usize, j); // Swap elements
            }
        }
        // Place the pivot in its correct position
        A.swap((i + 1) as usize, r);
        (i + 1) as usize // Return the index of the pivot
    }
}

// Function to print the array A with the pivot highlighted
fn print_array(p: usize, r: usize) {
    unsafe {
        for i in p..=r {
            if i == r {
                print!("[{}] ", A[i]); // Print pivot in brackets
            } else {
                print!("{} ", A[i]); // Print other elements normally
            }
        }
        println!(); // New line after printing the array
    }
}

// Example usage (not part of the specification, just for demonstration)
fn main() {
    unsafe {
        // Example initialization of the array A
        A[0] = 3;
        A[1] = 5;
        A[2] = 2;
        A[3] = 1;
        A[4] = 4;

        let p = 0;
        let r = 4;

        // Call partition and print the result
        let pivot_index = partition(p, r);
        print_array(p, r);
        println!("Pivot index: {}", pivot_index);
    }
}
