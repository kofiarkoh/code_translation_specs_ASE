#include <stdio.h>
#include <stdlib.h>

#define GRID_MIN 0
#define GRID_MAX 800
#define INITIAL_X 50
#define INITIAL_Y 0
#define EXIT_X 100
#define EXIT_Y 50

typedef struct {
    char orientation; // 'x' for vertical, 'y' for horizontal
    int x;
    int y;
} Mirror;

int main() {
    int n;
    // Read the number of mirrors
    if (scanf("%d", &n) != 1 || n < 0) {
        return 0; // Terminate gracefully on invalid input
    }

    Mirror mirrors[n]; // Array to hold mirror information

    // Read mirror data
    for (int i = 0; i < n; i++) {
        scanf(" %c %d %d", &mirrors[i].orientation, &mirrors[i].x, &mirrors[i].y);
    }

    // Initial position and direction
    int x = INITIAL_X;
    int y = INITIAL_Y;
    int dx = 1; // Moving right
    int dy = 0; // Not moving vertically

    while (1) {
        // Check for exit condition
        if (x == EXIT_X && y == EXIT_Y && dx == 1 && dy == 0) {
            break; // Exit the loop if the exit condition is met
        }

        // Update position
        x += dx;
        y += dy;

        // Check boundaries
        if (x < GRID_MIN || x > GRID_MAX || y < GRID_MIN || y > GRID_MAX) {
            break; // Stop movement if out of bounds
        }

        // Check for mirror interactions
        for (int i = 0; i < n; i++) {
            if ((mirrors[i].orientation == 'x' && x == mirrors[i].x && y == mirrors[i].y) || 
                (mirrors[i].orientation == 'y' && y == mirrors[i].y && x == mirrors[i].x)) {
                // Reflect based on mirror orientation
                if (mirrors[i].orientation == 'x') {
                    dx = -dx; // Change horizontal direction
                } else if (mirrors[i].orientation == 'y') {
                    dy = -dy; // Change vertical direction
                }
                break; // Exit the mirror check loop after reflection
            }
        }
    }

    // Output the final coordinates
    printf("%d %d\n", x, y);
    return 0;
}
