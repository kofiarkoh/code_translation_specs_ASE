fn explode(field: &mut [[char; 10]; 10], px: usize, py: usize) {
    // Check if the coordinates are within the valid range
    if px < 1 || px > 8 || py < 1 || py > 8 {
        return; // Undefined behavior if out of bounds, do nothing
    }

    // Helper function to perform the explosion recursively
    fn trigger_explosion(field: &mut [[char; 10]; 10], x: usize, y: usize) {
        // Check bounds to prevent out-of-bounds access
        if x >= 10 || y >= 10 {
            return; // Out of bounds, do nothing
        }
        // If the current cell is '1', change it to '0' (exploded)
        if field[y][x] == '1' {
            field[y][x] = '0'; // Mark as exploded
            // Recursively trigger explosions in adjacent cells
            trigger_explosion(field, x.wrapping_sub(1), y); // Left
            trigger_explosion(field, x + 1, y); // Right
            trigger_explosion(field, x, y.wrapping_sub(1)); // Up
            trigger_explosion(field, x, y + 1); // Down
        }
    }

    // Start the explosion from the given coordinates
    trigger_explosion(field, px, py);

    // Print the modified grid after the explosion
    for row in field.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn main() {
    // Example usage of the explode function
    let mut field: [[char; 10]; 10] = [
        ['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ['1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
    ];

    // Trigger an explosion at a valid position
    explode(&mut field, 4, 4);
}
