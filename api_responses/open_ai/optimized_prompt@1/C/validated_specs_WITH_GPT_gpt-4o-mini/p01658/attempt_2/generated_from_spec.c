#include <stdio.h>

int count_modified_ones(int M, int N, int matrix[M][N]) {
    int count = 0;

    // Process each row of the matrix
    for (int i = 0; i < M; i++) {
        for (int j = 0; j < N; j++) {
            // For the first row, count '1's directly
            if (i == 0) {
                if (matrix[i][j] == 1) {
                    count++; // Count the '1's in the first row
                }
            } else {
                // For subsequent rows, flip the value using XOR with 1
                int flipped_value = matrix[i][j] ^ 1; // Flip 0 to 1 and 1 to 0
                if (flipped_value == 1) {
                    count++; // Count the modified '1's in subsequent rows
                }
            }
        }
    }

    return count; // Return the total count of modified '1's
}

int main() {
    int M, N;
    // Read M and N
    scanf("%d %d", &M, &N);
    
    // Declare the matrix
    int matrix[M][N];

    // Read the matrix values
    for (int i = 0; i < M; i++) {
        for (int j = 0; j < N; j++) {
            scanf("%d", &matrix[i][j]);
        }
    }

    // Get the count of modified '1's
    int result = count_modified_ones(M, N, matrix);
    
    // Output the result
    printf("%d\n", result);
    
    return 0;
}
