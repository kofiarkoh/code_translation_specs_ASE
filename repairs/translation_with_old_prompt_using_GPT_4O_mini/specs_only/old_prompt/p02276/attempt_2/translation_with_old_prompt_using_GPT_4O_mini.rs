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

// Main function to demonstrate the partitioning
fn main() {
    // Example initialization of the array A
    unsafe {
        A[0] = 3;
        A[1] = 5;
        A[2] = 2;
        A[3] = 1;
        A[4] = 4;
    }

    let p = 0; // Starting index
    let r = 4; // Ending index

    let pivot_index = partition(p, r); // Perform partitioning
    print_array(p, r); // Print the array with the pivot highlighted
    println!("Pivot index: {}", pivot_index); // Print the pivot index
}
