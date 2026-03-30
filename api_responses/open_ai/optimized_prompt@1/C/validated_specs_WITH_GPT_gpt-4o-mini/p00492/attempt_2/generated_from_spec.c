#include <stdio.h>

#define MAX_HEIGHT 100 // Assuming a maximum height for the grid
#define MAX_WIDTH 100  // Assuming a maximum width for the grid

// Function to count adjacent cells to cells marked as "2"
int count_adjacent_cells(int grid[MAX_HEIGHT][MAX_WIDTH], int h, int w) {
    int count = 0;
    int directions[6][2] = {
        {1, 0},  // Right
        {0, 1},  // Down
        {-1, 0}, // Left
        {0, -1}, // Up
        {1, 1},  // Down-Right
        {-1, -1} // Up-Left
    };

    // Traverse the grid to mark cells
    for (int i = 0; i < h; i++) {
        for (int j = 0; j < (2 * w + 4); j++) {
            if (grid[i][j] == 2) {
                // Mark adjacent cells
                for (int d = 0; d < 6; d++) {
                    int ni = i + directions[d][0];
                    int nj = j + directions[d][1];
                    // Check bounds and mark if within grid
                    if (ni >= 0 && ni < h && nj >= 0 && nj < (2 * w + 4) && grid[ni][nj] != 2) {
                        grid[ni][nj] = 2; // Marking adjacent cells
                    }
                }
            }
        }
    }

    // Count cells adjacent to marked cells
    for (int i = 0; i < h; i++) {
        for (int j = 0; j < (2 * w + 4); j++) {
            if (grid[i][j] == 2) {
                for (int d = 0; d < 6; d++) {
                    int ni = i + directions[d][0];
                    int nj = j + directions[d][1];
                    // Check bounds and count if within grid
                    if (ni >= 0 && ni < h && nj >= 0 && nj < (2 * w + 4) && grid[ni][nj] != 2) {
                        count++;
                        grid[ni][nj] = 2; // Mark counted cells to avoid double counting
                    }
                }
            }
        }
    }

    return count; // Return the total count of adjacent cells
}

int main() {
    int w, h;
    int grid[MAX_HEIGHT][MAX_WIDTH];

    // Read width and height
    scanf("%d %d", &w, &h);

    // Read the grid
    for (int i = 0; i < h; i++) {
        for (int j = 0; j < (2 * w + 4); j++) {
            scanf("%d", &grid[i][j]);
        }
    }

    // Calculate and print the count of adjacent cells
    int result = count_adjacent_cells(grid, h, w);
    printf("%d\n", result);

    return 0;
}
