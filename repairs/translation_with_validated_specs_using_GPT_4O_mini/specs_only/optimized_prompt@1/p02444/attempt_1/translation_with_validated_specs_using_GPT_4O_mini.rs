fn rotate_array(n: usize, arr: &mut Vec<i32>, q: usize, queries: &[(usize, usize, usize)]) {
    for &(b, m, e) in queries {
        // Calculate the effective rotation
        let segment_length = e - b;
        let effective_rotation = if segment_length == 0 { 0 } else { m % segment_length };

        // Only perform rotation if effective_rotation is non-zero
        if effective_rotation > 0 {
            // Create a temporary vector to hold the rotated segment
            let mut temp = arr[b..e].to_vec();
            // Perform the rotation
            for i in 0..segment_length {
                temp[(i + effective_rotation) % segment_length] = arr[b + i];
            }
            // Copy the rotated segment back to the original array
            arr[b..e].copy_from_slice(&temp);
        }
    }
}

fn main() {
    // Example usage
    let n = 5;
    let mut arr = vec![1, 2, 3, 4, 5];
    let q = 2;
    let queries = vec![(0, 2, 5), (1, 3, 4)];

    rotate_array(n, &mut arr, q, &queries);
    
    // Output the modified array
    println!("{}", arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
