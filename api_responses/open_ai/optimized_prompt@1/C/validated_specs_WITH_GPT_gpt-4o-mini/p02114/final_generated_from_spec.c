#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 100 // Assuming a maximum grid size for simplicity

// Function prototypes
void mark_component(char grid[MAX_SIZE][MAX_SIZE], int n, int x, int y);
int count_connected_components(char grid[MAX_SIZE][MAX_SIZE], int n);

int main() {
    int n;
    char grid[MAX_SIZE][MAX_SIZE];

    // Read the size of the grid
    scanf("%d", &n);

    // Read the grid characters
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            scanf(" %c", &grid[i][j]); // Space before %c to skip whitespace
        }
    }

    // Count distinct connected components of 'x' and divide by 3
    int component_count = count_connected_components(grid, n);
    printf("%d\n", component_count / 3); // Output the result

    return 0;
}

// Function to mark all 'x' characters in the connected component
void mark_component(char grid[MAX_SIZE][MAX_SIZE], int n, int x, int y) {
    // Base case for recursion
    if (x < 0 || x >= n || y < 0 || y >= n || grid[x][y] != 'x') {
        return;
    }

    // Mark the current 'x' as visited (using a different character)
    grid[x][y] = 'o'; // Marking as 'o' to indicate visited

    // Recursively mark all adjacent 'x' characters
    mark_component(grid, n, x + 1, y); // Down
    mark_component(grid, n, x - 1, y); // Up
    mark_component(grid, n, x, y + 1); // Right
    mark_component(grid, n, x, y - 1); // Left
}

// Function to count distinct connected components of 'x'
int count_connected_components(char grid[MAX_SIZE][MAX_SIZE], int n) {
    int count = 0;

    // Iterate through the grid
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            // If we find an 'x', we have a new component
            if (grid[i][j] == 'x') {
                count++; // Increment component count
                mark_component(grid, n, i, j); // Mark the entire component
            }
        }
    }

    return count; // Return the total count of components
}
