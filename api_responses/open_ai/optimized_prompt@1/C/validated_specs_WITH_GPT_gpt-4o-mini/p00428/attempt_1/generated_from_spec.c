#include <stdio.h>
#include <stdlib.h>

int main() {
    int n, m;

    // Continue processing until termination condition (0, 0) is met
    while (1) {
        // Read dimensions of the matrix
        scanf("%d %d", &n, &m);
        if (n == 0 && m == 0) {
            break; // Termination condition
        }

        // Allocate memory for the matrix and column sums
        int **matrix = (int **)malloc(n * sizeof(int *));
        int *column_sums = (int *)calloc(m, sizeof(int));
        int *indices = (int *)malloc(m * sizeof(int));

        for (int i = 0; i < n; i++) {
            matrix[i] = (int *)malloc(m * sizeof(int));
            // Read the matrix row
            for (int j = 0; j < m; j++) {
                scanf("%d", &matrix[i][j]);
                column_sums[j] += matrix[i][j]; // Accumulate column sums
            }
        }

        // Initialize indices for sorting
        for (int j = 0; j < m; j++) {
            indices[j] = j + 1; // 1-based index
        }

        // Sort indices based on column sums in descending order using a simple bubble sort
        for (int j = 0; j < m - 1; j++) {
            for (int k = 0; k < m - j - 1; k++) {
                if (column_sums[indices[k] - 1] < column_sums[indices[k + 1] - 1]) {
                    // Swap indices
                    int temp = indices[k];
                    indices[k] = indices[k + 1];
                    indices[k + 1] = temp;
                }
            }
        }

        // Output the sorted indices
        for (int j = 0; j < m; j++) {
            printf("%d", indices[j]);
            if (j < m - 1) {
                printf(" "); // Print space between indices
            }
        }
        printf("\n");

        // Free allocated memory
        for (int i = 0; i < n; i++) {
            free(matrix[i]);
        }
        free(matrix);
        free(column_sums);
        free(indices);
    }

    return 0; // Graceful termination
}
