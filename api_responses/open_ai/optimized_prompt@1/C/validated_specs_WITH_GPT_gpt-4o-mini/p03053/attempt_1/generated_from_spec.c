#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MAX_GRID_SIZE 100 // Assuming a maximum grid size for simplicity

// Function to compute the maximum distance from placeholder to the nearest specific character
int max_distance_to_character(int height, int width, char grid[MAX_GRID_SIZE][MAX_GRID_SIZE], char specific_char, char placeholder) {
    int distances[MAX_GRID_SIZE][MAX_GRID_SIZE];
    int max_distance = 0;

    // Initialize distances
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            if (grid[i][j] == specific_char) {
                distances[i][j] = 0; // Distance to itself is zero
            } else {
                distances[i][j] = INT_MAX; // Initialize to a predefined maximum value
            }
        }
    }

    // Update distances based on neighboring cells
    int updated;
    do {
        updated = 0;
        for (int i = 0; i < height; i++) {
            for (int j = 0; j < width; j++) {
                if (grid[i][j] == placeholder) {
                    // Check all 4 possible directions (up, down, left, right)
                    if (i > 0 && distances[i - 1][j] < distances[i][j]) {
                        distances[i][j] = distances[i - 1][j] + 1;
                        updated = 1;
                    }
                    if (i < height - 1 && distances[i + 1][j] < distances[i][j]) {
                        distances[i][j] = distances[i + 1][j] + 1;
                        updated = 1;
                    }
                    if (j > 0 && distances[i][j - 1] < distances[i][j]) {
                        distances[i][j] = distances[i][j - 1] + 1;
                        updated = 1;
                    }
                    if (j < width - 1 && distances[i][j + 1] < distances[i][j]) {
                        distances[i][j] = distances[i][j + 1] + 1;
                        updated = 1;
                    }
                }
            }
        }
    } while (updated); // Repeat until no updates occur

    // Find the maximum distance from any placeholder cell
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            if (grid[i][j] == placeholder && distances[i][j] != INT_MAX) {
                if (distances[i][j] > max_distance) {
                    max_distance = distances[i][j];
                }
            }
        }
    }

    return max_distance; // Return the maximum distance found
}

int main() {
    int height, width;
    char grid[MAX_GRID_SIZE][MAX_GRID_SIZE];
    char specific_char = 'X'; // Example specific character
    char placeholder = '.'; // Example placeholder

    // Example input (to be replaced with actual input handling)
    height = 3;
    width = 3;
    grid[0][0] = 'X'; grid[0][1] = '.'; grid[0][2] = 'X';
    grid[1][0] = '.'; grid[1][1] = '.'; grid[1][2] = '.';
    grid[2][0] = 'X'; grid[2][1] = '.'; grid[2][2] = 'X';

    int result = max_distance_to_character(height, width, grid, specific_char, placeholder);
    printf("Maximum distance: %d\n", result); // Output the result

    return 0;
}
