#include <stdio.h>
#include <stdlib.h>

typedef struct HashEntry {
    int key; // This will store the value of V
    int L;   // Start of the range
    int R;   // End of the range
    struct HashEntry* next; // Pointer to the next entry for collision handling
} HashEntry;

#define HASH_SIZE 100 // Define a fixed size for the hash table

HashEntry* hashTable[HASH_SIZE] = {NULL}; // Initialize hash table

// Hash function to compute index for a given key
int hash(int key) {
    return (key % HASH_SIZE + HASH_SIZE) % HASH_SIZE; // Ensure non-negative index
}

// Function to insert or update the hash table
void insertOrUpdate(int V, int L, int R) {
    int index = hash(V);
    HashEntry* entry = hashTable[index];

    while (entry != NULL) {
        if (entry->key == V) {
            // Merge overlapping ranges
            if (L < entry->L) entry->L = L;
            if (R > entry->R) entry->R = R;
            return;
        }
        entry = entry->next;
    }

    // Create a new entry if not found
    HashEntry* newEntry = (HashEntry*)malloc(sizeof(HashEntry));
    newEntry->key = V;
    newEntry->L = L;
    newEntry->R = R;
    newEntry->next = hashTable[index];
    hashTable[index] = newEntry; // Insert at the head
}

// Function to compute the final count based on inputs
int computeCount(int N, int X, int D) {
    if (X == 0 && D == 0) return 1; // Both X and D are zero
    if (D == 0) return N + 1; // D is zero
    if (X == 0) return (N * (N - 1)) / 2 + 1; // X is zero and D is non-zero

    int count = 0;

    for (int i = 1; i <= N; i++) {
        int L = X + i; // Example calculation for L
        int R = X + i + D; // Example calculation for R
        int V = L % D; // Calculate V based on L and D

        insertOrUpdate(V, L, R); // Update the hash table with L and R
    }

    // Count unique ranges in the hash table
    for (int i = 0; i < HASH_SIZE; i++) {
        HashEntry* entry = hashTable[i];
        while (entry != NULL) {
            count++; // Count unique entries
            entry = entry->next;
        }
    }

    return count; // Return the final count
}

int main() {
    int N, X, D;
    // Assume valid input is provided for N, X, and D
    scanf("%d %d %d", &N, &X, &D);
    
    int result = computeCount(N, X, D);
    printf("%d\n", result); // Output the final count

    // Free allocated memory for hash table entries
    for (int i = 0; i < HASH_SIZE; i++) {
        HashEntry* entry = hashTable[i];
        while (entry != NULL) {
            HashEntry* temp = entry;
            entry = entry->next;
            free(temp);
        }
    }

    return 0;
}
