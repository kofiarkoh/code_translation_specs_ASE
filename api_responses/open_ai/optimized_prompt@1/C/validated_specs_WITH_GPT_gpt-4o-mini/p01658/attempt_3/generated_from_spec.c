#include <stdio.h>

int count_modified_ones(int M, int N, int matrix[M][N]) {
    int total_count = 0;

    for (int i = 0; i < M; i++) {
        // For the first row, count '1's directly
        // For subsequent rows, flip the values first
        if (i > 0) {
            for (int j = 0; j < N; j++) {
                matrix[i][j] ^= 1; // Flip 0s to 1s and 1s to 0s
            }
        }

        // Count '1's by traversing from both ends towards the center
        for (int j = 0; j < N / 2; j++) {
            // Check left and right elements
            if (matrix[i][j] == 1) {
                total_count++; // Count modified '1'
                matrix[i][j] = 0; // Modify the value
            }
            if (matrix[i][N - 1 - j] == 1) {
                total_count++; // Count modified '1'
                matrix[i][N - 1 - j] = 0; // Modify the value
            }
        }

        // If N is odd, check the middle element
        if (N % 2 == 1 && matrix[i][N / 2] == 1) {
            total_count++; // Count modified '1'
            matrix[i][N / 2] = 0; // Modify the value
        }
    }

    return total_count; // Return the total count of modified '1's
}

int main() {
    int M, N;
    // Read M and N
    scanf("%d %d", &M, &N);
    
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
