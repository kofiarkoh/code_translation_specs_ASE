#include <stdio.h>
#include <stdlib.h>

#define MAX_WIDTH 100 // Assuming a maximum width for the grid
#define MAX_HEIGHT 100 // Assuming a maximum height for the grid

// Directions for adjacent cells (up, down, left, right, and two diagonals)
int directions[6][2] = {
    {-1, 0}, // up
    {1, 0},  // down
    {0, -1}, // left
    {0, 1},  // right
    {-1, -1}, // up-left
    {1, 1}   // down-right
};

void mark_cells(int **grid, int h, int w) {
    // Traverse the grid to mark cells based on the presence of "2"
    for (int i = 0; i < h; i++) {
        for (int j = 0; j < 2 * w + 4; j++) {
            if (grid[i][j] == 2) {
                // Mark adjacent cells
                for (int d = 0; d < 6; d++) {
                    int ni = i + directions[d][0];
                    int nj = j + directions[d][1];
                    // Check bounds and mark if within the grid
                    if (ni >= 0 && ni < h && nj >= 0 && nj < 2 * w + 4 && grid[ni][nj] != 2) {
                        grid[ni][nj] = 2; // Mark the cell as "2"
                    }
                }
            }
        }
    }
}

int count_adjacent_cells(int **grid, int h, int w) {
    int count = 0;
    // Count cells adjacent to marked cells
    for (int i = 0; i < h; i++) {
        for (int j = 0; j < 2 * w + 4; j++) {
            if (grid[i][j] == 2) {
                // Check all adjacent cells
                for (int d = 0; d < 6; d++) {
                    int ni = i + directions[d][0];
                    int nj = j + directions[d][1];
                    // Check bounds
                    if (ni >= 0 && ni < h && nj >= 0 && nj < 2 * w + 4) {
                        // Count only if it's not already marked
                        if (grid[ni][nj] != 2) {
                            count++;
                            grid[ni][nj] = 2; // Mark as counted
                        }
                    }
                }
            }
        }
    }
    return count;
}

int main() {
    int w, h;
    // Read width and height
    scanf("%d %d", &w, &h);
    
    // Allocate memory for the grid
    int **grid = (int **)malloc(h * sizeof(int *));
    for (int i = 0; i < h; i++) {
        grid[i] = (int *)malloc((2 * w + 4) * sizeof(int));
    }

    // Read the grid values
    for (int i = 0; i < h; i++) {
        for (int j = 0; j < 2 * w + 4; j++) {
            scanf("%d", &grid[i][j]);
        }
    }

    // Mark cells based on the traversal
    mark_cells(grid, h, w);
    
    // Count adjacent cells
    int result = count_adjacent_cells(grid, h, w);
    
    // Output the result
    printf("%d\n", result);

    // Free allocated memory
    for (int i = 0; i < h; i++) {
        free(grid[i]);
    }
    free(grid);

    return 0;
}
