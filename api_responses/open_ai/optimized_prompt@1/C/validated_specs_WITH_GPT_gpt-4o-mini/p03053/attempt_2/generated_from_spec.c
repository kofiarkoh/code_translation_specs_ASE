#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MAX_SIZE 100 // Assuming a maximum grid size for simplicity

typedef struct {
    int x, y;
} Point;

int maxDistance(char grid[MAX_SIZE][MAX_SIZE], int height, int width, char specificChar) {
    int distances[MAX_SIZE][MAX_SIZE];
    int queue[MAX_SIZE * MAX_SIZE][2]; // Queue for BFS
    int front = 0, rear = 0;
    int maxDist = 0;

    // Initialize distances and enqueue specific character positions
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            if (grid[i][j] == specificChar) {
                distances[i][j] = 0; // Distance to itself is zero
                queue[rear][0] = i; // Enqueue row
                queue[rear][1] = j; // Enqueue column
                rear++;
            } else {
                distances[i][j] = INT_MAX; // Initialize other cells to max value
            }
        }
    }

    // Directions for moving in the grid (up, down, left, right)
    int directions[4][2] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

    // BFS to calculate distances
    while (front < rear) {
        Point current = {queue[front][0], queue[front][1]};
        front++;

        for (int d = 0; d < 4; d++) {
            int newX = current.x + directions[d][0];
            int newY = current.y + directions[d][1];

            // Check bounds
            if (newX >= 0 && newX < height && newY >= 0 && newY < width) {
                // If the new cell is a placeholder and we found a shorter distance
                if (grid[newX][newY] != specificChar && distances[newX][newY] > distances[current.x][current.y] + 1) {
                    distances[newX][newY] = distances[current.x][current.y] + 1; // Update distance
                    queue[rear][0] = newX; // Enqueue new position
                    queue[rear][1] = newY;
                    rear++;
                }
            }
        }
    }

    // Find the maximum distance from any placeholder
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            if (grid[i][j] != specificChar && distances[i][j] != INT_MAX) {
                if (distances[i][j] > maxDist) {
                    maxDist = distances[i][j]; // Update max distance
                }
            }
        }
    }

    return maxDist; // Return the maximum distance found
}

int main() {
    int height, width;
    char grid[MAX_SIZE][MAX_SIZE];
    char specificChar;

    // Input height, width, specific character, and grid
    scanf("%d %d %c", &height, &width, &specificChar);
    for (int i = 0; i < height; i++) {
        scanf("%s", grid[i]);
    }

    // Calculate and print the maximum distance
    int result = maxDistance(grid, height, width, specificChar);
    printf("%d\n", result);

    return 0;
}
