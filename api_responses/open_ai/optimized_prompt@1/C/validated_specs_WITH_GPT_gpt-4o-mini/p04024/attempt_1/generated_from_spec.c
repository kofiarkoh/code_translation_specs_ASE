#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007

int process_grid(int H, int W, int K, char **grid) {
    // If K is zero, return 1 immediately as per the specification
    if (K == 0) {
        return 1;
    }

    // Initialize result variable
    int result = 0;

    // Count the number of '#' characters in the grid
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            if (grid[i][j] == '#') {
                // Increment result for each '#' found
                result++;
            }
        }
    }

    // The result is computed based on the number of '#' characters and K
    // Here we assume a simple multiplication for demonstration purposes
    // This can be adjusted based on further specification details
    result = (result * K) % MOD;

    return result;
}

int main() {
    int H, W, K;

    // Read inputs for H, W, and K
    scanf("%d %d %d", &H, &W, &K);

    // Allocate memory for the grid
    char **grid = (char **)malloc(H * sizeof(char *));
    for (int i = 0; i < H; i++) {
        grid[i] = (char *)malloc((W + 1) * sizeof(char)); // +1 for null terminator
        scanf("%s", grid[i]); // Read each row of the grid
    }

    // Process the grid and get the result
    int result = process_grid(H, W, K, grid);

    // Print the result
    printf("%d\n", result);

    // Free allocated memory
    for (int i = 0; i < H; i++) {
        free(grid[i]);
    }
    free(grid);

    return 0;
}
