#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

int smallest_unique_concatenation(int N, int* list) {
    // Precondition: N must be a positive integer and the list must contain at least two distinct integers.
    // Postcondition: The output is the smallest concatenated value from two distinct integers.
    
    // Initialize the smallest concatenated value to the maximum possible integer.
    int smallest_concat = INT_MAX;
    
    // Iterate through all pairs of distinct integers in the list.
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            // Ensure we are concatenating two distinct integers.
            if (i != j) {
                // Create strings for concatenation.
                char str1[12]; // Buffer to hold the string representation of the integer.
                char str2[12]; // Buffer for the second integer.
                
                // Convert integers to strings.
                snprintf(str1, sizeof(str1), "%d", list[i]);
                snprintf(str2, sizeof(str2), "%d", list[j]);
                
                // Concatenate the two strings.
                char concat[24]; // Buffer to hold the concatenated result.
                snprintf(concat, sizeof(concat), "%s%s", str1, str2);
                
                // Convert the concatenated string back to an integer.
                long long concat_value = atoll(concat); // Use atoll to handle large values.
                
                // Check if the concatenated value is less than the current smallest.
                if (concat_value < smallest_concat) {
                    smallest_concat = concat_value;
                }
            }
        }
    }
    
    // If no valid concatenation exists, return the maximum possible integer.
    return smallest_concat == INT_MAX ? INT_MAX : smallest_concat;
}

// Example usage
int main() {
    int list[] = {12, 34, 56};
    int result = smallest_unique_concatenation(3, list);
    printf("Smallest unique concatenation: %d\n", result);
    return 0;
}
