const N: usize = 1_000_000; // Maximum size of the array

// Global array A
static mut A: [i32; N] = [0; N]; // Initialized with zeros

/// Partitions the array A from index p to r based on the pivot value A[r].
/// Returns the index of the pivot after partitioning.
fn partition(p: usize, r: usize) -> usize {
    // Safety: We assume p and r are valid indices as per the specification.
    unsafe {
        let pivot = A[r]; // The pivot is the last element in the segment
        let mut i = p as isize - 1; // i starts before the first element

        for j in p..r {
            if A[j] <= pivot {
                i += 1; // Move the boundary for elements <= pivot
                A.swap(i as usize, j); // Swap elements
            }
        }
        // Place the pivot in its correct position
        A.swap((i + 1) as usize, r);
        (i + 1) as usize // Return the new index of the pivot
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
