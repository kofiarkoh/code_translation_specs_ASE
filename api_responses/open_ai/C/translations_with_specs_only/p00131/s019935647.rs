fn main() {
    use std::io::{self, BufRead};

    // Read the number of matrices to process
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for the number of matrices
    let num_matrices: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Process each matrix
    for _ in 0..num_matrices {
        let mut primary_matrix = [[0; 10]; 10];
        let mut output_matrix = [[0; 10]; 10];

        // Read the 10x10 matrix
        for i in 0..10 {
            let line = lines.next().unwrap().unwrap();
            let values: Vec<i32> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            primary_matrix[i].copy_from_slice(&values[..10]);
        }

        // Process the primary matrix and create the output matrix
        for i in 0..10 {
            for j in 0..10 {
                if primary_matrix[i][j] % 2 != 0 { // Check for odd values
                    output_matrix[i][j] = 1; // Set output to 1 for odd values
                    primary_matrix[i][j] = 0; // Modify primary matrix in place
                } else {
                    output_matrix[i][j] = 0; // Set output to 0 for even values
                }
            }
        }

        // Output the modified matrix
        for row in output_matrix.iter() {
            println!("{}", row.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
        }
    }
}
```

### Explanation of Choices:
- **Matrix Size**: The specification clearly states that matrices are 10x10, so the code uses fixed-size arrays.
- **Input Handling**: The program reads from standard input, expecting the first line to be the number of matrices followed by the matrix data.
- **Odd Value Check**: The check for odd values is done using the modulus operator, which is a direct translation of the specification's requirement.
- **In-place Modification**: The primary matrix is modified in place as specified, setting odd values to 0.
- **Output Matrix**: The output matrix is constructed based on the presence of odd values, ensuring it only contains 0s and 1s.
- **Error Handling**: The code assumes valid input as per the specification, with no additional error handling implemented since it was not specified.
