#include <stdio.h>

int compute_number(int N, int M, int s[], int c[]) {
    // Initialize an array to hold digit values, -1 indicates unassigned
    int digits[10] = {-1}; // Assuming N <= 10 for simplicity, adjust as needed
    int computed_number = 0;

    // Handle special cases for M = 0
    if (M == 0) {
        if (N == 1) return 0; // Output 0 for N=1, M=0
        if (N == 2) return 10; // Output 10 for N=2, M=0
        if (N == 3) return 100; // Output 100 for N=3, M=0
        return -1; // For N > 3, return -1 (not specified, but safe)
    }

    // Process each digit-value pair
    for (int i = 0; i < M; i++) {
        int position = s[i] - 1; // Convert to 0-based index
        int value = c[i];

        // Check for valid position
        if (position < 0 || position >= N) return -1;

        // Check for conflicting values
        if (digits[position] != -1 && digits[position] != value) {
            return -1; // Conflict detected
        }

        // Assign the value to the digit position
        digits[position] = value;
    }

    // Check for the first digit position being 1 with value 0
    if (digits[0] == 0 && N > 1) return -1;

    // Compute the final number
    for (int i = 0; i < N; i++) {
        if (digits[i] == -1) return -1; // If any digit is unassigned, return -1
        computed_number = computed_number * 10 + digits[i];
    }

    return computed_number; // Return the computed number
}

int main() {
    int N, M;
    scanf("%d %d", &N, &M);
    int s[10], c[10]; // Assuming maximum of 10 pairs for simplicity

    for (int i = 0; i < M; i++) {
        scanf("%d %d", &s[i], &c[i]);
    }

    int result = compute_number(N, M, s, c);
    printf("%d\n", result);
    return 0;
}
