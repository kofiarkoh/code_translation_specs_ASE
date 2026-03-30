#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 100 // Define a maximum size for the grid

// Function to compute the maximum sum of values collected by two entities
int maxSum(int grid[MAX_SIZE][MAX_SIZE], int width, int height) {
    // Create a 2D array to store the maximum sums at each cell
    int dp[MAX_SIZE][MAX_SIZE] = {0};

    // Initialize the starting point
    dp[0][0] = grid[0][0];

    // Fill the dp array based on valid movements
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            if (i == 0 && j == 0) continue; // Skip the starting cell

            // Calculate the maximum sum from the top or left cell
            int fromTop = (i > 0) ? dp[i - 1][j] : 0;
            int fromLeft = (j > 0) ? dp[i][j - 1] : 0;

            // Update the current cell with the maximum sum possible
            dp[i][j] = grid[i][j] + (fromTop > fromLeft ? fromTop : fromLeft);
        }
    }

    // The maximum sum collected by the two entities is at the bottom-right corner
    return dp[height - 1][width - 1];
}

int main() {
    int width, height;
    int grid[MAX_SIZE][MAX_SIZE];

    // Read grid dimensions
    if (scanf("%d %d", &width, &height) != 2 || width <= 0 || height <= 0 || width > MAX_SIZE || height > MAX_SIZE) {
        // If input is invalid, behavior is undefined as per specification
        return 1; // Exit with an error code
    }

    // Read grid values
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            if (scanf("%d", &grid[i][j]) != 1) {
                // If input format is incorrect, behavior is undefined as per specification
                return 1; // Exit with an error code
            }
        }
    }

    // Compute and output the maximum sum
    int result = maxSum(grid, width, height);
    printf("%d\n", result);

    return 0;
}
