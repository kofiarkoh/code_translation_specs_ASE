#include <stdio.h>
#include <stdlib.h>

#define MOD 998244353

typedef struct {
    int key;
    int identifier;
} DataPair;

// Comparator function for sorting
int compare(const void *a, const void *b) {
    return ((DataPair *)a)->key - ((DataPair *)b)->key;
}

// Function to process the data pairs and compute the result
long compute_result(DataPair *pairs, int N) {
    long result = 0;
    int current_identifier = 0;
    int current_key = pairs[0].key;
    int count = 0;

    // First pass: Sort the pairs
    qsort(pairs, N, sizeof(DataPair), compare);

    // Second pass: Group by keys and update identifiers
    for (int i = 0; i < N; i++) {
        if (pairs[i].key != current_key) {
            // Update the result with contributions from the previous key group
            result = (result + (long)current_key * current_identifier % MOD * count) % MOD;
            current_key = pairs[i].key;
            current_identifier++;
            count = 0; // Reset count for new key
        }
        count++; // Count occurrences of the current key
        pairs[i].identifier = current_identifier; // Update identifier
    }
    // Final contribution for the last key group
    result = (result + (long)current_key * current_identifier % MOD * count) % MOD;

    return result;
}

int main() {
    int N;
    scanf("%d", &N); // Read number of data pairs

    DataPair *pairs = malloc(N * sizeof(DataPair));
    for (int i = 0; i < N; i++) {
        scanf("%d %d", &pairs[i].key, &pairs[i].identifier); // Read each key-identifier pair
    }

    long result = compute_result(pairs, N);
    printf("%ld\n", result); // Output the final result

    free(pairs); // Free allocated memory
    return 0;
}
