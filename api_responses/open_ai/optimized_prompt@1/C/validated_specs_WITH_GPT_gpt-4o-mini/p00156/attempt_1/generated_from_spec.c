#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MAX_SIZE 100 // Assuming a maximum grid size for simplicity

int min_transitions(int w, int h, char grid[MAX_SIZE][MAX_SIZE], char start_char) {
    if (w == 0 || h == 0) {
        // If dimensions are zero, terminate without output
        return -1; // Indicating no output
    }

    int start_x = -1, start_y = -1;

    // Find the starting position of the designated character
    for (int i = 0; i < h; i++) {
        for (int j = 0; j < w; j++) {
            if (grid[i][j] == start_char) {
                start_x = j;
                start_y = i;
                break;
            }
        }
        if (start_x != -1) break; // Break outer loop if found
    }

    if (start_x == -1) {
        // If the starting character is not found, return -1
        return -1; // Indicating no output
    }

    // Directions for adjacent cells (left, right, up, down)
    int directions[4][2] = {{0, -1}, {0, 1}, {-1, 0}, {1, 0}};
    int queue[MAX_SIZE * MAX_SIZE][2]; // Queue for BFS
    int front = 0, rear = 0;
    int visited[MAX_SIZE][MAX_SIZE] = {0}; // Visited cells
    int transitions[MAX_SIZE][MAX_SIZE] = {0}; // Transition counts

    // Initialize BFS
    queue[rear][0] = start_x;
    queue[rear][1] = start_y;
    rear++;
    visited[start_y][start_x] = 1;

    // BFS to find minimum transitions to any edge
    while (front < rear) {
        int x = queue[front][0];
        int y = queue[front][1];
        front++;

        // Check if we are at the edge of the grid
        if (x == 0 || x == w - 1 || y == 0 || y == h - 1) {
            return transitions[y][x] / 2; // Return half the transitions
        }

        // Explore adjacent cells
        for (int i = 0; i < 4; i++) {
            int new_x = x + directions[i][0];
            int new_y = y + directions[i][1];

            // Check bounds
            if (new_x >= 0 && new_x < w && new_y >= 0 && new_y < h) {
                // Calculate transition count
                int new_transition = transitions[y][x] + (grid[new_y][new_x] != grid[y][x]);
                if (!visited[new_y][new_x] || new_transition < transitions[new_y][new_x]) {
                    visited[new_y][new_x] = 1;
                    transitions[new_y][new_x] = new_transition;
                    queue[rear][0] = new_x;
                    queue[rear][1] = new_y;
                    rear++;
                }
            }
        }
    }

    return -1; // If no edge is reachable, return -1
}

int main() {
    int w, h;
    char grid[MAX_SIZE][MAX_SIZE];
    char start_char;

    // Read input dimensions
    scanf("%d %d", &w, &h);
    // Read the grid
    for (int i = 0; i < h; i++) {
        scanf("%s", grid[i]);
    }
    // Read the starting character
    scanf(" %c", &start_char);

    int result = min_transitions(w, h, grid, start_char);
    if (result != -1) {
        printf("%d\n", result);
    }

    return 0;
}
