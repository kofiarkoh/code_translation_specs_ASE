#include <stdio.h>
#include <stdlib.h>

typedef struct HashEntry {
    int v; // Value for hashing
    int l; // Left boundary
    int r; // Right boundary
    struct HashEntry* next; // Pointer to the next entry for collision resolution
} HashEntry;

#define HASH_SIZE 1000 // Define a fixed size for the hash table

HashEntry* hashTable[HASH_SIZE] = {NULL}; // Initialize hash table

// Hash function to compute index
int hash(int v) {
    return (v % HASH_SIZE + HASH_SIZE) % HASH_SIZE; // Ensure non-negative index
}

// Function to insert or merge ranges in the hash table
void insertOrMerge(int v, int l, int r) {
    int index = hash(v);
    HashEntry* entry = hashTable[index];

    while (entry != NULL) {
        if (entry->v == v) { // Found existing entry
            // Merge ranges if they overlap
            if (l <= entry->r + 1 && r >= entry->l - 1) {
                entry->l = entry->l < l ? entry->l : l; // Update left boundary
                entry->r = entry->r > r ? entry->r : r; // Update right boundary
            }
            return; // Exit after merging
        }
        entry = entry->next;
    }

    // Create a new entry if no existing entry found
    HashEntry* newEntry = (HashEntry*)malloc(sizeof(HashEntry));
    newEntry->v = v;
    newEntry->l = l;
    newEntry->r = r;
    newEntry->next = hashTable[index];
    hashTable[index] = newEntry; // Insert new entry
}

// Function to compute the final count from the hash table
int computeCount() {
    int count = 0;
    for (int i = 0; i < HASH_SIZE; i++) {
        HashEntry* entry = hashTable[i];
        while (entry != NULL) {
            count += (entry->r - entry->l) / D; // Count complete intervals
            entry = entry->next;
        }
    }
    return count;
}

int main() {
    int N, X, D;
    scanf("%d %d %d", &N, &X, &D); // Read inputs

    if (N < 0) return 0; // Invalid N, exit

    if (X == 0 && D == 0) {
        printf("1\n"); // Postcondition for X and D both zero
        return 0;
    }

    if (D == 0) {
        printf("%d\n", N + 1); // Postcondition for D zero
        return 0;
    }

    if (X == 0) {
        printf("%d\n", N * (N - 1) / 2 + 1); // Postcondition for X zero and D non-zero
        return 0;
    }

    // Main counting logic
    for (int i = 1; i <= N; i++) {
        int L = (X * 2 + D * (i - 1)) * i / 2; // Calculate L
        int R = (X * 2 - D * (i - 1)) * i / 2 + D; // Calculate R
        int V = L % D; // Calculate V

        insertOrMerge(V, L, R); // Insert or merge in hash table
    }

    int finalCount = computeCount(); // Compute final count
    printf("%d\n", finalCount); // Output the final count

    return 0;
}
