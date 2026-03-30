#include <stdio.h>

#define SIZE 3

int main() {
    int matrix[SIZE][SIZE];
    int row_min[SIZE];
    int normalized[SIZE][SIZE];
    int i, j;

    // Read the 3x3 matrix from standard input
    for (i = 0; i < SIZE; i++) {
        for (j = 0; j < SIZE; j++) {
            if (scanf("%d", &matrix[i][j]) != 1) {
                // If input is not as expected, terminate with no output
                return 0;
            }
        }
    }

    // Compute the minimum value of each row
    for (i = 0; i < SIZE; i++) {
        row_min[i] = matrix[i][0];
        for (j = 1; j < SIZE; j++) {
            if (matrix[i][j] < row_min[i]) {
                row_min[i] = matrix[i][j];
            }
        }
    }

    // Normalize the matrix by subtracting the row minimums
    for (i = 0; i < SIZE; i++) {
        for (j = 0; j < SIZE; j++) {
            normalized[i][j] = matrix[i][j] - row_min[i];
        }
    }

    // Check if all columns in the normalized matrix are identical
    int identical = 1; // Assume columns are identical initially
    for (j = 0; j < SIZE; j++) {
        for (i = 1; i < SIZE; i++) {
            if (normalized[i][j] != normalized[0][j]) {
                identical = 0; // Found a difference, columns are not identical
                break;
            }
        }
        if (!identical) {
            break; // No need to check further
        }
    }

    // Output the result based on column equality
    if (identical) {
        printf("Yes\n");
    } else {
        printf("No\n");
    }

    return 0;
}
