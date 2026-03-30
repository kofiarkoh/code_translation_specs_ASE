#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 205

// Function to compute the maximum sum of values collected by two entities
int maxSum(int grid[MAX_SIZE][MAX_SIZE], int width, int height) {
    // Create a 2D array to store the maximum sums
    int dp[MAX_SIZE][MAX_SIZE][MAX_SIZE][MAX_SIZE] = {0};

    // Initialize the starting point
    dp[0][0][0][0] = grid[0][0];

    // Iterate through all possible positions for both entities
    for (int x1 = 0; x1 < height; x1++) {
        for (int y1 = 0; y1 < width; y1++) {
            for (int x2 = 0; x2 < height; x2++) {
                for (int y2 = 0; y2 < width; y2++) {
                    // Only consider valid positions
                    if (x1 == x2 && y1 == y2) {
                        // If both entities are in the same cell, add the value only once
                        dp[x1][y1][x2][y2] += grid[x1][y1];
                    } else {
                        // If they are in different cells, sum their values
                        dp[x1][y1][x2][y2] += grid[x1][y1] + grid[x2][y2];
                    }

                    // Move down or right for both entities
                    if (x1 + 1 < height) {
                        dp[x1 + 1][y1][x2][y2] = 
                            (dp[x1 + 1][y1][x2][y2] > dp[x1][y1][x2][y2]) ? 
                            dp[x1 + 1][y1][x2][y2] : dp[x1][y1][x2][y2];
                    }
                    if (y1 + 1 < width) {
                        dp[x1][y1 + 1][x2][y2] = 
                            (dp[x1][y1 + 1][x2][y2] > dp[x1][y1][x2][y2]) ? 
                            dp[x1][y1 + 1][x2][y2] : dp[x1][y1][x2][y2];
                    }
                    if (x2 + 1 < height) {
                        dp[x1][y1][x2 + 1][y2] = 
                            (dp[x1][y1][x2 + 1][y2] > dp[x1][y1][x2][y2]) ? 
                            dp[x1][y1][x2 + 1][y2] : dp[x1][y1][x2][y2];
                    }
                    if (y2 + 1 < width) {
                        dp[x1][y1][x2][y2 + 1] = 
                            (dp[x1][y1][x2][y2 + 1] > dp[x1][y1][x2][y2]) ? 
                            dp[x1][y1][x2][y2 + 1] : dp[x1][y1][x2][y2];
                    }
                }
            }
        }
    }

    // The result is the maximum sum at the bottom-right corner
    return dp[height - 1][width - 1][height - 1][width - 1];
}

int main() {
    int width, height;
    int grid[MAX_SIZE][MAX_SIZE];

    // Read grid dimensions
    scanf("%d %d", &width, &height);

    // Read grid values
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            scanf("%d", &grid[i][j]);
        }
    }

    // Compute and print the maximum sum
    int result = maxSum(grid, width, height);
    printf("%d\n", result);

    return 0;
}
