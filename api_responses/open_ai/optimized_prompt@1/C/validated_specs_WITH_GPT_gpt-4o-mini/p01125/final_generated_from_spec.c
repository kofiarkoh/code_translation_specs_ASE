#include <stdio.h>
#include <string.h>

#define GRID_SIZE 25
#define MAX_OCCUPIED 100

// Directions for movement
#define UP 'U'
#define DOWN 'D'
#define LEFT 'L'
#define RIGHT 'R'

// Structure to represent a cell
typedef struct {
    int x;
    int y;
} Cell;

int main() {
    int occupied_count;
    Cell occupied_cells[MAX_OCCUPIED];
    char direction;
    int steps;
    int current_x = 10, current_y = 10; // Initial position
    int visited[MAX_OCCUPIED] = {0}; // Track visited occupied cells
    int visited_count = 0;

    // Read number of occupied cells
    scanf("%d", &occupied_count);
    
    // Read occupied cell coordinates
    for (int i = 0; i < occupied_count; i++) {
        scanf("%d %d", &occupied_cells[i].x, &occupied_cells[i].y);
    }

    // Read movement commands
    while (scanf(" %c %d", &direction, &steps) == 2) {
        for (int step = 0; step < steps; step++) {
            // Update position based on direction
            if (direction == UP) {
                current_y--;
            } else if (direction == DOWN) {
                current_y++;
            } else if (direction == LEFT) {
                current_x--;
            } else if (direction == RIGHT) {
                current_x++;
            }

            // Check if the new position is within bounds
            if (current_x < 0 || current_x >= GRID_SIZE || current_y < 0 || current_y >= GRID_SIZE) {
                // Out of bounds, ignore this step
                continue;
            }

            // Check if the current position corresponds to an occupied cell
            for (int i = 0; i < occupied_count; i++) {
                if (occupied_cells[i].x == current_x && occupied_cells[i].y == current_y && !visited[i]) {
                    visited[i] = 1; // Mark as visited
                    visited_count++;
                    break; // No need to check other cells
                }
            }
        }
    }

    // Check if all occupied cells have been visited
    if (visited_count == occupied_count) {
        printf("Yes\n");
    } else {
        printf("No\n");
    }

    return 0;
}
