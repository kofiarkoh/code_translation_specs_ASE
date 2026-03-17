use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the input into a 3x3 matrix
    let mut matrix = [[0; 3]; 3];
    let mut values = input.split_whitespace().filter_map(|s| s.parse::<i32>().ok());
    
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] = values.next().expect("Expected exactly 9 integers");
        }
    }

    // Compute the minimum value for each row
    let mut min = [0; 3];
    for i in 0..3 {
        min[i] = *matrix[i].iter().min().expect("Row should not be empty");
    }

    // Normalize the matrix by subtracting the row minimums
    for i in 0..3 {
        for j in 0..3 {
            matrix[i][j] -= min[i];
        }
    }

    // Check if all columns in the normalized matrix are equal
    for j in 0..3 {
        if matrix[0][j] != matrix[1][j] || matrix[0][j] != matrix[2][j] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
```

### Notes:
1. **Input Handling**: The Rust code reads from standard input and expects a single line of input containing nine integers. It uses `filter_map` to safely parse integers, ensuring that only valid integers are considered.
2. **Matrix Initialization**: The matrix is initialized as a 3x3 array, and values are filled in a nested loop.
3. **Minimum Calculation**: The minimum value for each row is computed using the `iter().min()` method, which is idiomatic and safe.
4. **Normalization**: The normalization step is straightforward, subtracting the row minimums from each element.
5. **Column Equality Check**: The check for column equality is performed in a loop, and the output is printed accordingly.
6. **Error Handling**: The program assumes valid input as per the specification and does not handle cases where fewer than nine integers are provided. This aligns with the specification's precondition.
