#include <stdio.h>

int compute_number(int N, int M, int s[], int c[]) {
    // Initialize an array to hold digit values, -1 indicates unassigned
    int digits[10] = {-1}; // Assuming N <= 10 for simplicity
    int i;

    // Handle special cases for M = 0
    if (M == 0) {
        if (N == 1) return 0; // Output 0 for 1 digit
        if (N == 2) return 10; // Output 10 for 2 digits
        if (N == 3) return 100; // Output 100 for 3 digits
        return -1; // For N > 3, return -1 (not specified, but safe)
    }

    // Process each digit-value pair
    for (i = 0; i < M; i++) {
        int pos = s[i] - 1; // Convert to 0-based index
        int value = c[i];

        // Check for position validity
        if (pos < 0 || pos >= N) return -1;

        // Check for conflicting values
        if (digits[pos] != -1 && digits[pos] != value) {
            return -1; // Conflict detected
        }

        // Assign the value to the digit position
        digits[pos] = value;
    }

    // Check for the first digit being zero in a multi-digit number
    if (digits[0] == 0 && N > 1) {
        return -1; // Invalid case
    }

    // Compute the final number
    int result = 0;
    for (i = 0; i < N; i++) {
        // If the position is unassigned, treat it as 0
        if (digits[i] == -1) {
            digits[i] = 0;
        }
        result = result * 10 + digits[i]; // Build the number
    }

    return result; // Return the computed number
}

int main() {
    // Example usage
    int N = 3;
    int M = 2;
    int s[] = {1, 3};
    int c[] = {5, 7};
    
    int result = compute_number(N, M, s, c);
    printf("%d\n", result); // Expected output: 507

    return 0;
}
