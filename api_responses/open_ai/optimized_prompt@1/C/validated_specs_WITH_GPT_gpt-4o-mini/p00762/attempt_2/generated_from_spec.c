#include <stdio.h>

#define MAX_ORIENTATIONS 6
#define GRID_SIZE 10 // Assuming a fixed grid size for the height and top maps

// Function to initialize the height and top maps
void initialize_maps(int height_map[GRID_SIZE][GRID_SIZE], int top_map[GRID_SIZE][GRID_SIZE]) {
    for (int i = 0; i < GRID_SIZE; i++) {
        for (int j = 0; j < GRID_SIZE; j++) {
            height_map[i][j] = 0; // Initialize height to 0
            top_map[i][j] = 0;    // Initialize top orientation to 0
        }
    }
}

// Function to process block placements
void process_block_placements(int placements, int orientations[][2], int height_map[GRID_SIZE][GRID_SIZE], int top_map[GRID_SIZE][GRID_SIZE], int orientation_count[MAX_ORIENTATIONS + 1]) {
    for (int p = 0; p < placements; p++) {
        int top_orientation = orientations[p][0];
        int front_orientation = orientations[p][1];

        // Assume a fixed position for simplicity; in a real scenario, this would be dynamic
        int x = 0; // Current x position
        int y = 0; // Current y position

        // Check if the block can be placed
        if (height_map[x][y] < height_map[x][y] + 1) { // Simplified condition for placement
            height_map[x][y]++; // Increment height
            top_map[x][y] = top_orientation; // Set top orientation
            orientation_count[top_orientation]++; // Increment count for the top orientation
        }
    }
}

// Main function
int main() {
    int height_map[GRID_SIZE][GRID_SIZE];
    int top_map[GRID_SIZE][GRID_SIZE];
    int orientation_count[MAX_ORIENTATIONS + 1] = {0}; // Count for orientations 1 to 6

    // Initialize maps
    initialize_maps(height_map, top_map);

    // Read number of placements
    int placements;
    scanf("%d", &placements);

    // Read orientations
    int orientations[placements][2];
    for (int i = 0; i < placements; i++) {
        scanf("%d %d", &orientations[i][0], &orientations[i][1]);
    }

    // Process block placements
    process_block_placements(placements, orientations, height_map, top_map, orientation_count);

    // Output the counts for each orientation
    for (int i = 1; i <= MAX_ORIENTATIONS; i++) {
        printf("%d ", orientation_count[i]);
    }
    printf("\n");

    return 0;
}
