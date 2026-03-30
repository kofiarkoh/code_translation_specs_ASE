#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define INF INT_MAX // Define INF as the maximum integer value

typedef struct {
    int x, y;
} Point;

int maxDistance(char **grid, int height, int width) {
    int maxDist = 0;
    int **distances = (int **)malloc(height * sizeof(int *));
    for (int i = 0; i < height; i++) {
        distances[i] = (int *)malloc(width * sizeof(int));
        for (int j = 0; j < width; j++) {
            if (grid[i][j] == '#') {
                distances[i][j] = 0; // Distance to itself is zero
            } else {
                distances[i][j] = INF; // Initialize other cells to INF
            }
        }
    }

    // BFS setup
    Point *queue = (Point *)malloc(height * width * sizeof(Point));
    int front = 0, rear = 0;

    // Enqueue all specific character positions
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            if (grid[i][j] == '#') {
                queue[rear++] = (Point){i, j}; // Add to queue
            }
        }
    }

    // Directions for moving in the grid (up, down, left, right)
    int directions[4][2] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

    // BFS processing
    while (front < rear) {
        Point current = queue[front++];
        for (int d = 0; d < 4; d++) {
            int newX = current.x + directions[d][0];
            int newY = current.y + directions[d][1];
            // Check bounds
            if (newX >= 0 && newX < height && newY >= 0 && newY < width) {
                // If it's a placeholder and we can update its distance
                if (grid[newX][newY] == '.' && distances[newX][newY] == INF) {
                    distances[newX][newY] = distances[current.x][current.y] + 1; // Update distance
                    queue[rear++] = (Point){newX, newY}; // Enqueue the new position
                    if (distances[newX][newY] > maxDist) {
                        maxDist = distances[newX][newY]; // Update max distance
                    }
                }
            }
        }
    }

    // Free allocated memory
    for (int i = 0; i < height; i++) {
        free(distances[i]);
    }
    free(distances);
    free(queue);

    return maxDist; // Return the maximum distance found
}

int main() {
    // Example usage
    int height = 3, width = 3;
    char *grid[] = {
        "#..",
        "...",
        "..#"
    };

    int result = maxDistance(grid, height, width);
    printf("%d\n", result); // Output the maximum distance
    return 0;
}
