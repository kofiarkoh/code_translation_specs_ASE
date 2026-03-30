#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MODULO 1000000007

// Function to convert a character string to an integer
int string_to_int(const char *str) {
    int result = 0;
    while (*str) {
        result = result * 10 + (*str - '0');
        str++;
    }
    return result;
}

// Memoization structure
typedef struct {
    int position;
    int is_lower_bound;
    int is_upper_bound;
} MemoKey;

typedef struct {
    MemoKey key;
    int value;
} MemoEntry;

MemoEntry *memo = NULL;
int memo_size = 0;

// Function to check if a key exists in the memoization structure
int find_in_memo(MemoKey key) {
    for (int i = 0; i < memo_size; i++) {
        if (memo[i].key.position == key.position &&
            memo[i].key.is_lower_bound == key.is_lower_bound &&
            memo[i].key.is_upper_bound == key.is_upper_bound) {
            return memo[i].value;
        }
    }
    return -1; // Not found
}

// Function to add a key-value pair to the memoization structure
void add_to_memo(MemoKey key, int value) {
    memo = realloc(memo, sizeof(MemoEntry) * (memo_size + 1));
    memo[memo_size].key = key;
    memo[memo_size].value = value;
    memo_size++;
}

// Recursive function to count valid sequences
int count_sequences(const char *A, const char *B, const char *C, int pos, int is_lower_bound, int is_upper_bound) {
    if (pos == strlen(C)) {
        return 1; // Found a valid sequence
    }

    MemoKey key = {pos, is_lower_bound, is_upper_bound};
    int memo_result = find_in_memo(key);
    if (memo_result != -1) {
        return memo_result; // Return cached result
    }

    int count = 0;
    for (char digit = '0'; digit <= '9'; digit++) {
        // Check bounds
        int new_is_lower_bound = is_lower_bound && (digit == A[pos]);
        int new_is_upper_bound = is_upper_bound && (digit == B[pos]);

        // If within bounds, continue recursion
        if (!new_is_lower_bound || (new_is_lower_bound && digit >= A[pos])) {
            if (!new_is_upper_bound || (new_is_upper_bound && digit <= B[pos])) {
                count = (count + count_sequences(A, B, C, pos + 1, new_is_lower_bound, new_is_upper_bound)) % MODULO;
            }
        }
    }

    add_to_memo(key, count); // Cache the result
    return count;
}

// Main function to compute the number of valid sequences
int main(int argc, char *argv[]) {
    if (argc < 4) {
        return 1; // Error: fewer than three strings provided
    }

    const char *A = argv[1];
    const char *B = argv[2];
    const char *C = argv[3];

    // Pad A with leading zeros if necessary
    int lenA = strlen(A);
    int lenB = strlen(B);
    if (lenA < lenB) {
        char *paddedA = malloc(lenB + 1);
        memset(paddedA, '0', lenB - lenA);
        strcpy(paddedA + (lenB - lenA), A);
        A = paddedA; // Use the padded version
    }

    // Initialize memoization structure
    memo = NULL;
    memo_size = 0;

    // Count valid sequences
    int result = count_sequences(A, B, C, 0, 1, 1);

    // Free memoization structure
    free(memo);
    if (lenA < lenB) {
        free((void *)A); // Free the padded version
    }

    printf("%d\n", result); // Output the result
    return 0;
}
