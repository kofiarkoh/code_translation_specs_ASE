#include <stdio.h>
#include <limits.h>
#include <string.h>

int smallest_unique_concatenation(int N, int *list) {
    // Precondition: N must be a positive integer and at least 2 distinct integers are required.
    // Postcondition: Return the smallest unique concatenated value or max int if none found.
    
    // Initialize the smallest concatenated value to the maximum possible integer
    int smallest_concat = INT_MAX;
    
    // Iterate through all pairs of distinct integers
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            if (i != j) { // Ensure distinct integers
                // Create concatenated strings
                char concat1[20]; // Buffer to hold concatenated result
                char concat2[20]; // Buffer to hold concatenated result
                snprintf(concat1, sizeof(concat1), "%d%d", list[i], list[j]);
                snprintf(concat2, sizeof(concat2), "%d%d", list[j], list[i]);
                
                // Convert concatenated strings back to integers
                long long int value1 = atoll(concat1);
                long long int value2 = atoll(concat2);
                
                // Update smallest_concat if a smaller value is found
                if (value1 < smallest_concat) {
                    smallest_concat = value1;
                }
                if (value2 < smallest_concat) {
                    smallest_concat = value2;
                }
            }
        }
    }
    
    // If no valid concatenation was found, return the maximum possible integer
    return smallest_concat == INT_MAX ? INT_MAX : smallest_concat;
}

int main() {
    // Example usage
    int list[] = {12, 34, 56};
    int result = smallest_unique_concatenation(3, list);
    printf("Smallest unique concatenation: %d\n", result);
    return 0;
}
