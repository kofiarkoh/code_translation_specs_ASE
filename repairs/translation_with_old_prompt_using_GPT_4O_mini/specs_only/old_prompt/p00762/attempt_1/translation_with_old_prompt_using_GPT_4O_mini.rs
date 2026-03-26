const N_MAX: usize = 4; // Maximum grid size based on the specification

// Global state representing the height and top face of blocks in the grid
static mut HEIGHT_MAP: [[usize; N_MAX * 2 + 1]; N_MAX * 2 + 1] = [[0; N_MAX * 2 + 1]; N_MAX * 2 + 1];
static mut TOP_MAP: [[char; N_MAX * 2 + 1]; N_MAX * 2 + 1] = [['0'; N_MAX * 2 + 1]; N_MAX * 2 + 1];

fn place_blocks(num_blocks: usize, top: char, front: char) {
    // If no blocks to place, simply return
    if num_blocks == 0 {
        return;
    }

    // Convert char to usize for indexing
    let top_index = top.to_digit(10).unwrap() as usize;
    let front_index = front.to_digit(10).unwrap() as usize;

    // Update the height and top face in the grid
    unsafe {
        HEIGHT_MAP[top_index][front_index] += 1; // Increment height
        TOP_MAP[top_index][front_index] = top; // Set the top face
    }
}

fn print_block_counts() {
    let mut counts = [0; 6]; // Counts for block types 1 to 6

    unsafe {
        for i in 0..=N_MAX * 2 {
            for j in 0..=N_MAX * 2 {
                let top_face = TOP_MAP[i][j];
                if top_face >= '1' && top_face <= '6' {
                    let index = (top_face as usize) - ('1' as usize);
                    counts[index] += HEIGHT_MAP[i][j]; // Accumulate counts
                }
            }
        }
    }

    // Print counts separated by spaces
    println!("{}", counts.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "));
}
