#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MAX_SIZE 100 // Assuming a maximum grid size for simplicity

// Function to compute the minimum transitions to the edge of the grid
int minTransitionsToEdge(int w, int h, char grid[MAX_SIZE][MAX_SIZE], char startChar) {
    if (w == 0 || h == 0) {
        // If dimensions are zero, terminate without output
        return 0;
    }

    int minTransitions = INT_MAX;

    // Directions for moving in the grid: up, down, left, right
    int directions[4][2] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

    // Function to perform DFS and count transitions
    void dfs(int x, int y, int transitions, int visited[MAX_SIZE][MAX_SIZE]) {
        // Check if out of bounds
        if (x < 0 || x >= h || y < 0 || y >= w) {
            // If we reach the edge, update the minimum transitions
            if (transitions < minTransitions) {
                minTransitions = transitions;
            }
            return;
        }

        // If already visited, return
        if (visited[x][y]) {
            return;
        }

        // Mark the cell as visited
        visited[x][y] = 1;

        // Explore all four directions
        for (int i = 0; i < 4; i++) {
            int newX = x + directions[i][0];
            int newY = y + directions[i][1];
            int newTransitions = transitions;

            // Count transition if the character differs
            if (newX >= 0 && newX < h && newY >= 0 && newY < w) {
                if (grid[newX][newY] != grid[x][y]) {
                    newTransitions++;
                }
            }

            // Recursive DFS call
            dfs(newX, newY, newTransitions, visited);
        }

        // Unmark the cell for other paths
        visited[x][y] = 0;
    }

    // Find all starting positions and initiate DFS from each
    for (int i = 0; i < h; i++) {
        for (int j = 0; j < w; j++) {
            if (grid[i][j] == startChar) {
                int visited[MAX_SIZE][MAX_SIZE] = {0}; // Reset visited for each start
                dfs(i, j, 0, visited);
            }
        }
    }

    // Return the minimum transitions divided by two as per specification
    return (minTransitions == INT_MAX) ? 0 : minTransitions / 2;
}

int main() {
    int w, h;
    char grid[MAX_SIZE][MAX_SIZE];
    char startChar;

    // Read input dimensions and grid
    scanf("%d %d", &w, &h);
    for (int i = 0; i < h; i++) {
        scanf("%s", grid[i]);
    }
    scanf(" %c", &startChar); // Read the starting character

    // Compute and print the minimum transitions
    int result = minTransitionsToEdge(w, h, grid, startChar);
    printf("%d\n", result);

    return 0;
}
