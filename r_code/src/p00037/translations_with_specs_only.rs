fn main() {
    // Define a 5x5 array to hold the binary values
    let mut grid: [[u8; 5]; 5] = [[0; 5]; 5];
    
    // Read exactly 9 input strings
    let inputs: Vec<String> = std::iter::repeat_with(|| {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    })
    .take(9)
    .collect();

    // Update the 2D array based on the input strings
    for (i, input) in inputs.iter().enumerate() {
        for (j, char) in input.chars().enumerate() {
            if char == '1' {
                grid[i][j] = 1; // Set to 1 if the character is '1'
            }
        }
    }

    // Generate the output based on the state of the grid
    let mut output = String::new();
    let mut x = 0; // Starting position (row)
    let mut y = 0; // Starting position (column)

    // Traverse the grid to determine movement commands
    while x < 5 && y < 5 {
        if grid[x][y] == 1 {
            output.push('R'); // Move right if the current cell is 1
            y += 1;
        } else {
            output.push('D'); // Move down if the current cell is 0
            x += 1;
        }

        // Check for boundaries to prevent out-of-bounds access
        if x >= 5 || y >= 5 {
            break; // Exit if we reach the end of the grid
        }
    }

    // Output the result followed by a newline
    println!("{}", output);
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads exactly 9 strings, ensuring that the input is limited to the specified number.
2. **Grid Initialization**: A 5x5 grid is initialized to hold binary values, with each cell defaulting to 0.
3. **Updating the Grid**: The grid is updated based on the input strings, where '1' sets the corresponding cell to 1.
4. **Movement Logic**: The movement logic is based on the values in the grid, moving right for '1' and down for '0'. The loop checks for boundaries to avoid out-of-bounds access.
5. **Output**: The output is constructed as a string of movement commands and printed with a newline at the end, adhering to the specification's requirements.
