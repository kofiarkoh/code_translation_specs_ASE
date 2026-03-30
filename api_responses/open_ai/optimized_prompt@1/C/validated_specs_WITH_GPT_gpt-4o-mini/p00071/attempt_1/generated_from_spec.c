#include <stdio.h>

#define GRID_SIZE 10
#define EXPLOSION_SIZE 8

void explode(char grid[GRID_SIZE][GRID_SIZE], int x, int y) {
    // Check if the starting coordinates are within bounds
    if (x < 1 || x > EXPLOSION_SIZE || y < 1 || y > EXPLOSION_SIZE) {
        return; // Out of bounds, do nothing
    }

    // Adjust for 0-based indexing
    x--; 
    y--;

    // Set the starting cell to '0'
    grid[y + 1][x + 1] = '0'; // Adjusting for the 1-based input to 0-based grid

    // Use a stack for DFS to propagate the explosion effect
    int stack[64][2]; // Max 64 cells in an 8x8 grid
    int top = -1;

    // Push the starting cell onto the stack
    stack[++top][0] = y + 1; // Adjusted for 0-based grid
    stack[top][1] = x + 1;   // Adjusted for 0-based grid

    // Directions for adjacent cells (up, down, left, right)
    int directions[4][2] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

    while (top >= 0) {
        int curr_y = stack[top][0];
        int curr_x = stack[top--][1];

        // Check all adjacent cells
        for (int i = 0; i < 4; i++) {
            int new_y = curr_y + directions[i][0];
            int new_x = curr_x + directions[i][1];

            // Check bounds
            if (new_y >= 0 && new_y < GRID_SIZE && new_x >= 0 && new_x < GRID_SIZE) {
                // If the cell is '1', set it to '0' and push it onto the stack
                if (grid[new_y][new_x] == '1') {
                    grid[new_y][new_x] = '0';
                    stack[++top][0] = new_y; // Push new coordinates onto the stack
                    stack[top][1] = new_x;
                }
            }
        }
    }
}

int main() {
    int datasets;
    scanf("%d", &datasets); // Read the number of datasets

    for (int d = 0; d < datasets; d++) {
        char grid[GRID_SIZE][GRID_SIZE];

        // Read the 8x8 grid into the 10x10 grid (padding with '0's)
        for (int i = 0; i < GRID_SIZE; i++) {
            for (int j = 0; j < GRID_SIZE; j++) {
                if (i > 0 && i < EXPLOSION_SIZE + 1 && j > 0 && j < EXPLOSION_SIZE + 1) {
                    scanf(" %c", &grid[i][j]); // Read character input
                } else {
                    grid[i][j] = '0'; // Initialize padding with '0's
                }
            }
        }

        int start_x, start_y;
        scanf("%d %d", &start_x, &start_y); // Read starting coordinates

        // Process the explosion effect
        explode(grid, start_x, start_y);

        // Print the modified grid
        for (int i = 1; i < EXPLOSION_SIZE + 1; i++) {
            for (int j = 1; j < EXPLOSION_SIZE + 1; j++) {
                printf("%c", grid[i][j]);
            }
            printf("\n");
        }
    }

    return 0;
}
