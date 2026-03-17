// Constants for the maximum grid size
const N_MAX: usize = 4; // As per the specification, this can be adjusted if needed
const GRID_SIZE: usize = N_MAX * 2 + 1;

// Global state representing the height and top face of blocks in the grid
static mut HEIGHT_MAP: [[usize; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
static mut TOP_MAP: [[char; GRID_SIZE]; GRID_SIZE] = [['0'; GRID_SIZE]; GRID_SIZE];

// Function to place blocks on the grid
fn place_blocks(blocks: &[(char, char)]) {
    // Iterate over each block placement
    for &(top, front) in blocks {
        // Calculate the position to place the block (for simplicity, we place it at the center)
        let x = N_MAX; // Center x-coordinate
        let y = N_MAX; // Center y-coordinate

        // Update the height map and top map
        unsafe {
            HEIGHT_MAP[y][x] += 1; // Increment the height at the center
            TOP_MAP[y][x] = top; // Set the top face of the block
        }
    }

    // Count the number of blocks of each type (1 to 6)
    let mut counts = [0; 6];
    unsafe {
        for row in HEIGHT_MAP.iter() {
            for &height in row.iter() {
                if height > 0 {
                    let top_face = TOP_MAP[row.as_ptr() as usize][row.as_ptr() as usize]; // Get the top face
                    if top_face >= '1' && top_face <= '6' {
                        counts[(top_face as usize - '1' as usize)] += height; // Count the blocks
                    }
                }
            }
        }
    }

    // Print the counts of blocks of types 1 to 6
    println!("{}", counts.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "));
}

// Main function to simulate the block placement
fn main() {
    // Example input: a list of blocks to place
    let blocks = vec![('1', '2'), ('3', '4'), ('5', '6')];

    // Place the blocks on the grid
    place_blocks(&blocks);
}
