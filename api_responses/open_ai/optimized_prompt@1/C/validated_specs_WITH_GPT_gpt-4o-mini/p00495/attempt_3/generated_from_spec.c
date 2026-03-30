#include <stdio.h>

int longest_subsequence(int len1, int seq1[], int len2, int seq2[]) {
    // Check for valid lengths (precondition)
    if (len1 < 0 || len2 < 0) {
        // Undefined behavior as per specification
        return -1; // Indicating an error
    }

    int max_length = 0;

    // Iterate through each starting position in seq2
    for (int start = 0; start < len2; start++) {
        int i = 0; // Pointer for seq1
        int current_length = 0;

        // Iterate through seq2 starting from 'start'
        for (int j = start; j < len2; j++) {
            // Check if the current element in seq2 matches the current element in seq1
            if (i < len1 && seq2[j] == seq1[i]) {
                current_length++; // Found a match, increase the length
                i++; // Move to the next element in seq1
            }
            // If we have matched all elements in seq1, we can stop
            if (i >= len1) {
                break;
            }
        }

        // Update max_length if the current subsequence is longer
        if (current_length > max_length) {
            max_length = current_length;
        }
    }

    // Postcondition: max_length is non-negative and <= len2
    return max_length;
}

int main() {
    // Example usage
    int seq1[] = {1, 2, 3};
    int seq2[] = {1, 1, 2, 3, 1, 2, 3};
    int len1 = sizeof(seq1) / sizeof(seq1[0]);
    int len2 = sizeof(seq2) / sizeof(seq2[0]);

    int result = longest_subsequence(len1, seq1, len2, seq2);
    printf("Length of longest subsequence: %d\n", result);
    return 0;
}
