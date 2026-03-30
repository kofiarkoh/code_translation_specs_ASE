#include <stdio.h>

#define GRID_SIZE 10
#define DATASET_ROWS 8
#define DATASET_COLS 8

void explode(char grid[GRID_SIZE][GRID_SIZE], int x, int y) {
    // Check if the current position is within bounds and is '1'
    if (x < 1 || x > DATASET_ROWS || y < 1 || y > DATASET_COLS || grid[x][y] != '1') {
        return; // Out of bounds or not an explosion cell
    }

    // Set the current cell to '0' to mark it as exploded
    grid[x][y] = '0';

    // Recursively propagate the explosion in all four directions
    explode(grid, x - 1, y); // Up
    explode(grid, x + 1, y); // Down
    explode(grid, x, y - 1); // Left
    explode(grid, x, y + 1); // Right
}

void process_datasets(int num_datasets) {
    char grid[GRID_SIZE][GRID_SIZE];
    int start_x, start_y;

    for (int i = 0; i < num_datasets; i++) {
        // Read the grid for the current dataset
        for (int j = 1; j <= DATASET_ROWS; j++) {
            for (int k = 1; k <= DATASET_COLS; k++) {
                scanf(" %c", &grid[j][k]); // Read character input
            }
        }

        // Read starting coordinates for the explosion
        scanf("%d %d", &start_x, &start_y);

        // Trigger explosion if starting coordinates are valid
        explode(grid, start_x, start_y);

        // Print the modified grid
        for (int j = 1; j <= DATASET_ROWS; j++) {
            for (int k = 1; k <= DATASET_COLS; k++) {
                printf("%c", grid[j][k]);
            }
            printf("\n");
        }
    }
}

int main() {
    int num_datasets;

    // Read the number of datasets
    scanf("%d", &num_datasets);

    // Process each dataset
    process_datasets(num_datasets);

    return 0;
}
