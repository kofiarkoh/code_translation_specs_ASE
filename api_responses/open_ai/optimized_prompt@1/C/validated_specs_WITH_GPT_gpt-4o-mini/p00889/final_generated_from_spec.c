#include <stdio.h>
#include <stdlib.h>

#define HASH_TABLE_SIZE 1000 // Arbitrary size for the hash table

// Hash table entry structure
typedef struct HashEntry {
    int key;
    int value;
    struct HashEntry* next;
} HashEntry;

// Hash table structure
typedef struct HashTable {
    HashEntry** entries;
} HashTable;

// Function to create a hash table
HashTable* create_hash_table() {
    HashTable* table = malloc(sizeof(HashTable));
    table->entries = malloc(sizeof(HashEntry*) * HASH_TABLE_SIZE);
    for (int i = 0; i < HASH_TABLE_SIZE; i++) {
        table->entries[i] = NULL;
    }
    return table;
}

// Hash function
unsigned int hash(int key) {
    return key % HASH_TABLE_SIZE;
}

// Function to insert into the hash table
void hash_table_insert(HashTable* table, int key, int value) {
    unsigned int index = hash(key);
    HashEntry* new_entry = malloc(sizeof(HashEntry));
    new_entry->key = key;
    new_entry->value = value;
    new_entry->next = table->entries[index];
    table->entries[index] = new_entry;
}

// Function to lookup in the hash table
int hash_table_lookup(HashTable* table, int key) {
    unsigned int index = hash(key);
    HashEntry* entry = table->entries[index];
    while (entry != NULL) {
        if (entry->key == key) {
            return entry->value;
        }
        entry = entry->next;
    }
    return 0; // Return 0 if not found
}

// Function to free the hash table
void free_hash_table(HashTable* table) {
    for (int i = 0; i < HASH_TABLE_SIZE; i++) {
        HashEntry* entry = table->entries[i];
        while (entry != NULL) {
            HashEntry* temp = entry;
            entry = entry->next;
            free(temp);
        }
    }
    free(table->entries);
    free(table);
}

// Main processing function
int process_sequence(int n, int s, int w, int q) {
    HashTable* table = create_hash_table();
    int count = 0;
    int* sequence = malloc(n * sizeof(int));

    // Generate the sequence
    for (int i = 0; i < n; i++) {
        sequence[i] = s;
        s = (s + w) % q; // Transformation based on w and q
    }

    // Count specific patterns
    for (int i = 0; i < n; i++) {
        if (sequence[i] != 0) {
            count++; // Increment count for non-zero values
            int lookup_value = hash_table_lookup(table, sequence[i]);
            hash_table_insert(table, sequence[i], lookup_value + 1);
        }
    }

    free(sequence);
    free_hash_table(table);
    return count; // Return the total count
}

int main() {
    int n, s, w, q;

    // Read input until a non-positive n is encountered
    while (1) {
        scanf("%d", &n);
        if (n <= 0) break; // Terminate on non-positive n
        scanf("%d %d %d", &s, &w, &q);
        if (q <= 0) continue; // Skip invalid modulus

        int result = process_sequence(n, s, w, q);
        printf("%d\n", result); // Output the result
    }

    return 0;
}
