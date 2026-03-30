#include <stdio.h>

int compute_matching_value(int N, const char *S, const char *T) {
    // Initialize the index for the last matched character
    int i = -1; // No match found initially
    // Iterate from the end of the strings towards the beginning
    for (int j = N - 1; j >= 0; j--) {
        // Check if characters match
        if (S[j] == T[N - 1 - j]) {
            i = j; // Update the index of the last matched character
            break; // Exit the loop after the first match
        }
    }
    
    // If a match was found, calculate the result
    if (i != -1) {
        return 2 * N - (N - i - 1); // Calculate the result based on the last match
    }
    
    // If no match was found, return 0 (as per the postcondition of non-negative output)
    return 0;
}

int main() {
    // Example usage
    int N = 5;
    const char *S = "abcde";
    const char *T = "edcba";
    
    int result = compute_matching_value(N, S, T);
    printf("Result: %d\n", result); // Output the result
    return 0;
}
