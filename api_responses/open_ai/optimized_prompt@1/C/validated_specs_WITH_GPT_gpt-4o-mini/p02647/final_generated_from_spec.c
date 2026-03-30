#include <stdio.h>

void transform_array(int N, int K, int arr[]) {
    int B[N]; // Temporary array to track changes
    for (int i = 0; i < N; i++) {
        B[i] = 0; // Initialize B to zero
    }

    for (int iter = 0; iter < K; iter++) {
        // Reset B for each iteration
        for (int i = 0; i < N; i++) {
            B[i] = 0;
        }

        for (int i = 0; i < N; i++) {
            if (arr[i] >= i) {
                B[0]++; // Increment B[0] if arr[i] >= i
            }
            if (arr[i] < i) {
                int pos = i - arr[i];
                if (pos >= 0 && pos < N) {
                    B[pos]++; // Increment B at pos if arr[i] < i
                }
            }
            if (arr[i] < (N - i)) {
                int pos = i + arr[i];
                if (pos >= 0 && pos < N) {
                    B[pos]--; // Decrement B at pos if arr[i] < N - i
                }
            }
        }

        // Accumulate values from B into arr
        for (int i = 0; i < N; i++) {
            if (i > 0) {
                B[i] += B[i - 1]; // Accumulate changes
            }
            arr[i] += B[i]; // Update arr with accumulated values
            if (arr[i] < 0) {
                arr[i] = 0; // Ensure non-negativity
            }
        }

        // Check for termination condition
        int min_value = arr[0];
        for (int i = 1; i < N; i++) {
            if (arr[i] < min_value) {
                min_value = arr[i];
            }
        }
        if (min_value < N) {
            break; // Terminate if minimum value is less than N
        }
    }
}

int main() {
    int N, K;
    scanf("%d %d", &N, &K); // Read N and K
    int arr[N];
    for (int i = 0; i < N; i++) {
        scanf("%d", &arr[i]); // Read array elements
    }

    transform_array(N, K, arr); // Process the array

    // Print the transformed array
    for (int i = 0; i < N; i++) {
        printf("%d", arr[i]);
        if (i < N - 1) {
            printf(" "); // Print space between elements
        }
    }
    printf("\n"); // New line after printing the array
    return 0;
}
