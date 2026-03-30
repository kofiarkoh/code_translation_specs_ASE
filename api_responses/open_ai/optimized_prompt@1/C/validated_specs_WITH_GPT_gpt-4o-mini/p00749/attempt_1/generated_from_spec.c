#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_WIDTH 100
#define MAX_HEIGHT 100

// Directions for exploring connected components (up, down, left, right)
int directions[4][2] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

// Function to check if a cell is within the grid bounds
int is_within_bounds(int x, int y, int width, int height) {
    return (x >= 0 && x < width && y >= 0 && y < height);
}

// Function to perform DFS and find connected components
void dfs(char grid[MAX_HEIGHT][MAX_WIDTH], int visited[MAX_HEIGHT][MAX_WIDTH], 
         int x, int y, int *size, int *max_x) {
    visited[y][x] = 1; // Mark the cell as visited
    (*size)++; // Increment the size of the component
    if (x > *max_x) {
        *max_x = x; // Update the maximum x-coordinate
    }

    // Explore all four directions
    for (int i = 0; i < 4; i++) {
        int new_x = x + directions[i][0];
        int new_y = y + directions[i][1];
        if (is_within_bounds(new_x, new_y, MAX_WIDTH, MAX_HEIGHT) && 
            !visited[new_y][new_x] && grid[new_y][new_x] != ' ') {
            dfs(grid, visited, new_x, new_y, size, max_x);
        }
    }
}

// Function to evaluate the stability of the structure
const char* evaluate_stability(char grid[MAX_HEIGHT][MAX_WIDTH], int width, int height) {
    int visited[MAX_HEIGHT][MAX_WIDTH] = {0}; // Visited array to track explored cells
    int stable = 1; // Assume stable unless proven otherwise

    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            if (grid[y][x] != ' ' && !visited[y][x]) {
                int size = 0; // Size of the connected component
                int max_x = x; // Maximum x-coordinate in the component
                dfs(grid, visited, x, y, &size, &max_x);

                // Stability condition: if the maximum x-coordinate is less than the size
                if (max_x < size) {
                    stable = 0; // Mark as unstable
                }
            }
        }
    }

    return stable ? "STABLE" : "UNSTABLE"; // Return stability result
}

int main() {
    int width, height;
    char grid[MAX_HEIGHT][MAX_WIDTH];

    // Read grid dimensions
    scanf("%d %d", &width, &height);
    
    // Check for positive dimensions
    if (width <= 0 || height <= 0) {
        return 0; // Terminate without processing
    }

    // Read the grid
    for (int y = 0; y < height; y++) {
        scanf("%s", grid[y]);
    }

    // Evaluate and print the stability of the structure
    printf("%s\n", evaluate_stability(grid, width, height));

    return 0;
}
