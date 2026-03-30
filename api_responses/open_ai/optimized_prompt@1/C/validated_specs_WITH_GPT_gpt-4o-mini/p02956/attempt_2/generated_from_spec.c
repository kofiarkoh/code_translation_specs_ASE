#include <stdio.h>
#include <stdlib.h>

#define MOD 998244353

typedef struct {
    int key;
    int identifier;
} DataPair;

// Comparator function for sorting DataPair by key
int compare(const void *a, const void *b) {
    return ((DataPair *)a)->key - ((DataPair *)b)->key;
}

// Function to process the data pairs and compute the result
long compute_result(DataPair *pairs, int N) {
    long result = 0;
    int current_identifier = 0;
    
    // Sort the data pairs by their keys
    qsort(pairs, N, sizeof(DataPair), compare);
    
    // Grouping and updating identifiers
    for (int i = 0; i < N; i++) {
        if (i == 0 || pairs[i].key != pairs[i - 1].key) {
            current_identifier++; // Increment identifier for new key
        }
        pairs[i].identifier = current_identifier; // Update identifier
    }
    
    // Compute the final result based on updated identifiers
    for (int i = 0; i < N; i++) {
        // Aggregate contributions from each key-identifier pair
        result = (result + (long)pairs[i].key * pairs[i].identifier) % MOD;
    }
    
    return result;
}

int main() {
    int N;
    scanf("%d", &N); // Read the number of data pairs

    // Allocate memory for data pairs
    DataPair *pairs = (DataPair *)malloc(N * sizeof(DataPair));
    
    // Read the data pairs
    for (int i = 0; i < N; i++) {
        scanf("%d %d", &pairs[i].key, &pairs[i].identifier);
    }

    // Compute the result
    long result = compute_result(pairs, N);
    
    // Output the result
    printf("%ld\n", result);
    
    // Free allocated memory
    free(pairs);
    
    return 0;
}
