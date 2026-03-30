#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MODULO 1000000007 // Predefined constant for output modulo

// Function to convert a character to its integer value
int charToInt(char c) {
    return c - '0'; // Assumes valid numeric character input
}

// Recursive function to count valid sequences
int countValidSequences(const char *A, const char *B, const char *C, int posC, int lowerBound, int upperBound, int **memo) {
    // Base case: if we have processed all characters in C
    if (posC == strlen(C)) {
        return 1; // Found a valid sequence
    }

    // Check memoization
    if (memo[posC][lowerBound] != -1) {
        return memo[posC][lowerBound];
    }

    int count = 0;
    int targetDigit = charToInt(C[posC]);

    // Explore all digits from lowerBound to upperBound
    for (int digit = lowerBound; digit <= upperBound; digit++) {
        if (digit == targetDigit) {
            count = (count + countValidSequences(A, B, C, posC + 1, lowerBound, upperBound, memo)) % MODULO;
        }
    }

    // Store result in memoization structure
    memo[posC][lowerBound] = count;
    return count;
}

// Main function to compute the number of valid sequences
int computeValidSequences(const char *A, const char *B, const char *C) {
    int lenC = strlen(C);
    int lenA = strlen(A);
    int lenB = strlen(B);

    // Allocate memoization structure
    int **memo = (int **)malloc(lenC * sizeof(int *));
    for (int i = 0; i < lenC; i++) {
        memo[i] = (int *)malloc((lenB + 1) * sizeof(int)); // Upper bound can be at most lenB
        memset(memo[i], -1, (lenB + 1) * sizeof(int)); // Initialize with -1
    }

    // Start counting valid sequences
    int result = countValidSequences(A, B, C, 0, charToInt(A[0]), charToInt(B[0]), memo);

    // Free allocated memory
    for (int i = 0; i < lenC; i++) {
        free(memo[i]);
    }
    free(memo);

    return result; // Return the count of valid sequences
}

// Entry point of the program
int main(int argc, char *argv[]) {
    if (argc < 4) {
        return 1; // Terminate with error code if fewer than three strings are provided
    }

    // Compute the number of valid sequences
    int result = computeValidSequences(argv[1], argv[2], argv[3]);
    printf("%d\n", result); // Output the result

    return 0;
}
