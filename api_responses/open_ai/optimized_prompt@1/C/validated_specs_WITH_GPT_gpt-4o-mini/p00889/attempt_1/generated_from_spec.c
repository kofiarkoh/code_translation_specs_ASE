#include <stdio.h>
#include <stdlib.h>

#define HASH_TABLE_SIZE 1000 // Arbitrary size for the hash table

typedef struct HashNode {
    int key;
    int value;
    struct HashNode* next;
} HashNode;

typedef struct HashTable {
    HashNode** table;
} HashTable;

// Function to create a hash table
HashTable* createHashTable() {
    HashTable* ht = malloc(sizeof(HashTable));
    ht->table = malloc(sizeof(HashNode*) * HASH_TABLE_SIZE);
    for (int i = 0; i < HASH_TABLE_SIZE; i++) {
        ht->table[i] = NULL;
    }
    return ht;
}

// Hash function
unsigned int hash(int key) {
    return key % HASH_TABLE_SIZE;
}

// Function to insert or update the hash table
void insert(HashTable* ht, int key, int value) {
    unsigned int index = hash(key);
    HashNode* node = ht->table[index];
    
    while (node != NULL) {
        if (node->key == key) {
            node->value += value; // Update existing key
            return;
        }
        node = node->next;
    }
    
    // Create a new node if key not found
    HashNode* newNode = malloc(sizeof(HashNode));
    newNode->key = key;
    newNode->value = value;
    newNode->next = ht->table[index];
    ht->table[index] = newNode;
}

// Function to count patterns based on input parameters
int countPatterns(int n, int s, int w, int q) {
    HashTable* ht = createHashTable();
    int count = 0;

    for (int i = 0; i < n; i++) {
        int transformedValue = (s + i * w) % q; // Transformation based on input parameters
        insert(ht, transformedValue, 1); // Insert into hash table
        count++; // Increment count for each valid transformation
    }

    // Clean up hash table
    for (int i = 0; i < HASH_TABLE_SIZE; i++) {
        HashNode* node = ht->table[i];
        while (node != NULL) {
            HashNode* temp = node;
            node = node->next;
            free(temp);
        }
    }
    free(ht->table);
    free(ht);

    return count; // Return the total count of patterns
}

int main() {
    int n, s, w, q;

    while (1) {
        scanf("%d", &n); // Read the length of the sequence
        if (n <= 0) break; // Terminate on non-positive n
        scanf("%d %d %d", &s, &w, &q); // Read the other parameters

        if (q <= 0) continue; // Skip processing if q is not positive

        int result = countPatterns(n, s, w, q); // Compute the count
        printf("%d\n", result); // Output the result
    }

    return 0;
}
