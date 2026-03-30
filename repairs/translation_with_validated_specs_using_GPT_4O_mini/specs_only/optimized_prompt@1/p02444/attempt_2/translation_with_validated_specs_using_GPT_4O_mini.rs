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
    // Read input values
    let n: usize = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    
    let mut arr: Vec<i32> = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
    };
    
    let q: usize = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    
    let mut queries = Vec::new();
    for _ in 0..q {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let query: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        queries.push((query[0], query[1], query[2]));
    }

    rotate_array(n, &mut arr, q, &queries);
    
    // Output the modified array
    println!("{}", arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
