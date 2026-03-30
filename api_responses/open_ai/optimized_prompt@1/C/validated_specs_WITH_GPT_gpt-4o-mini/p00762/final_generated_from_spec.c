#include <stdio.h>

#define GRID_SIZE 10 // Assuming a fixed grid size for height and top maps
#define ORIENTATION_COUNT 6

// Function to initialize the height and top maps
void initialize_maps(int height_map[GRID_SIZE][GRID_SIZE], int top_map[GRID_SIZE][GRID_SIZE]) {
    for (int i = 0; i < GRID_SIZE; i++) {
        for (int j = 0; j < GRID_SIZE; j++) {
            height_map[i][j] = 0; // Initialize height to 0
            top_map[i][j] = 0;    // Initialize top orientation to 0
        }
    }
}

// Function to check if a block can be placed at the given position
int can_place_block(int height_map[GRID_SIZE][GRID_SIZE], int top_map[GRID_SIZE][GRID_SIZE], 
                    int x, int y, int top_orientation, int front_orientation) {
    // Check if the height of the target position is less than the current position
    if (height_map[x][y] >= height_map[x][y]) {
        return 0; // Cannot place block if height is not less
    }
    // Check orientation conditions
    if (front_orientation > 3) {
        // Check adjacent positions (up, down, left, right)
        if ((x > 0 && top_map[x-1][y] <= front_orientation) || 
            (x < GRID_SIZE-1 && top_map[x+1][y] <= front_orientation) || 
            (y > 0 && top_map[x][y-1] <= front_orientation) || 
            (y < GRID_SIZE-1 && top_map[x][y+1] <= front_orientation)) {
            return 1; // Can place block
        }
    }
    return 0; // Cannot place block
}

// Main function to process block placements
int main() {
    int height_map[GRID_SIZE][GRID_SIZE];
    int top_map[GRID_SIZE][GRID_SIZE];
    int orientation_count[ORIENTATION_COUNT + 1] = {0}; // Count for orientations 1 to 6
    int num_placements;

    initialize_maps(height_map, top_map);

    // Read number of block placements
    scanf("%d", &num_placements);
    
    for (int i = 0; i < num_placements; i++) {
        int top_orientation, front_orientation;
        scanf("%d %d", &top_orientation, &front_orientation);

        // Assuming a fixed position for simplicity; this can be modified as needed
        int x = 0, y = 0; // Example fixed position

        // Check if the block can be placed
        if (can_place_block(height_map, top_map, x, y, top_orientation, front_orientation)) {
            height_map[x][y]++; // Increment height at position
            top_map[x][y] = top_orientation; // Update top orientation
            orientation_count[top_orientation]++; // Increment count for the orientation
        }
    }

    // Output the counts for orientations 1 to 6
    for (int i = 1; i <= ORIENTATION_COUNT; i++) {
        printf("%d", orientation_count[i]);
        if (i < ORIENTATION_COUNT) {
            printf(" "); // Space-separated output
        }
    }
    printf("\n");

    return 0;
}
