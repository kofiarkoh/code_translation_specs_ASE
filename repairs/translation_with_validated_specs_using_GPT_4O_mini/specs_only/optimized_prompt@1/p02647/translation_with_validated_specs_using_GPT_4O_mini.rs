fn transform_array(n: usize, k: usize, mut arr: Vec<usize>) -> Vec<usize> {
    // Create a temporary array B initialized to zero
    let mut b = vec![0; n];

    // Perform up to K iterations
    for _ in 0..k {
        // Reset B for each iteration
        b.fill(0);

        // Apply transformation logic
        for (i, &value) in arr.iter().enumerate() {
            if value >= i {
                // Increment B at the first position
                b[0] += 1;
            }
            if value < i {
                // Increment B at the position determined by index - value
                let pos = i - value;
                if pos < n {
                    b[pos] += 1;
                }
            }
            if value < n - i {
                // Decrement B at the position determined by index + value
                let pos = i + value;
                if pos < n {
                    b[pos] = b[pos].wrapping_sub(1); // Use wrapping_sub to avoid negative values
                }
            }
        }

        // Accumulate values from B into arr
        let mut accumulated = 0;
        for j in 0..n {
            accumulated += b[j];
            arr[j] = arr[j].wrapping_sub(accumulated); // Use wrapping_sub to avoid negative values
            if arr[j] < n {
                // If any value is less than N, we can terminate early
                break;
            }
        }
    }

    arr
}

fn main() {
    let n = 5; // Example size of the array
    let k = 3; // Example number of iterations
    let arr = vec![10, 2, 3, 4, 5]; // Example input array

    let transformed_array = transform_array(n, k, arr);
    println!("{:?}", transformed_array);
}
