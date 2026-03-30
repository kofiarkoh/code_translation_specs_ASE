#include <stdio.h>

int main() {
    int A, B, C, D;
    // Read four integers as input
    scanf("%d %d %d %d", &A, &B, &C, &D);

    // Initialize the count of overlapping integers
    int overlap_count = 0;

    // Calculate the overlapping range
    int start = (A > C) ? A : C; // Start of the overlap
    int end = (B < D) ? B : D;   // End of the overlap

    // Check if there is an overlap
    if (start <= end) {
        // Count the number of overlapping integers
        overlap_count = end - start + 1; // Inclusive count
    }

    // Adjust the count by subtracting one if there is at least one overlap
    if (overlap_count > 0) {
        overlap_count -= 1;
    }

    // Output the result
    printf("%d\n", overlap_count);

    return 0;
}
