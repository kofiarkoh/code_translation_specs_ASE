#include <stdio.h>

#define MAX_H 51
#define MAX_W 60

int main() {
    int H, W;
    char grid[MAX_H][MAX_W];

    // Read the dimensions of the grid
    scanf("%d %d", &H, &W);

    // Read the grid characters
    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            scanf(" %c", &grid[i][j]); // Space before %c to skip whitespace
        }
    }

    // Check adjacency of '#' characters
    int all_adjacent = 1; // Assume all '#' are adjacent until proven otherwise

    // Directions for adjacent cells (up, down, left, right)
    int directions[4][2] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

    for (int i = 0; i < H; i++) {
        for (int j = 0; j < W; j++) {
            if (grid[i][j] == '#') {
                int has_adjacent = 0; // Flag to check if this '#' has an adjacent '#'
                for (int d = 0; d < 4; d++) {
                    int ni = i + directions[d][0];
                    int nj = j + directions[d][1];
                    // Check bounds and adjacency
                    if (ni >= 0 && ni < H && nj >= 0 && nj < W && grid[ni][nj] == '#') {
                        has_adjacent = 1; // Found an adjacent '#'
                        break; // No need to check further
                    }
                }
                if (!has_adjacent) {
                    all_adjacent = 0; // Found a '#' without adjacent '#'
                    break; // No need to check further
                }
            }
        }
        if (!all_adjacent) {
            break; // Exit outer loop if condition is already false
        }
    }

    // Output result based on adjacency check
    if (all_adjacent) {
        printf("Yes\n");
    } else {
        printf("No\n");
    }

    return 0;
}
