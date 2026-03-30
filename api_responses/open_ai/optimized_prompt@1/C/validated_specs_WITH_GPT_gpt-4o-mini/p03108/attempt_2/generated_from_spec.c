#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *parent; // Array to hold the parent of each node
    int *size;   // Array to hold the size of each component
    long long *results; // Array to hold the results for each operation
    int n;       // Number of nodes
    int m;       // Number of union operations
    int current; // Current index for results
    long long disconnected_pairs; // Count of disconnected pairs
} UnionFind;

// Function to initialize the union-find structure
UnionFind* init_union_find(int n) {
    UnionFind *uf = malloc(sizeof(UnionFind));
    uf->parent = malloc(n * sizeof(int));
    uf->size = malloc(n * sizeof(int));
    uf->results = malloc(n * sizeof(long long));
    uf->n = n;
    uf->m = 0;
    uf->current = 0;
    uf->disconnected_pairs = (long long)n * (n - 1) / 2; // Initial count of disconnected pairs

    for (int i = 0; i < n; i++) {
        uf->parent[i] = i; // Each node is its own parent
        uf->size[i] = 1;   // Each component size is 1 initially
    }
    return uf;
}

// Find function with path compression
int find(UnionFind *uf, int x) {
    if (uf->parent[x] != x) {
        uf->parent[x] = find(uf, uf->parent[x]); // Path compression
    }
    return uf->parent[x];
}

// Union function with union by size
void union_nodes(UnionFind *uf, int x, int y) {
    int rootX = find(uf, x);
    int rootY = find(uf, y);

    if (rootX != rootY) {
        // If they are in different components, merge them
        uf->disconnected_pairs -= (long long)uf->size[rootX] * uf->size[rootY]; // Update disconnected pairs
        if (uf->size[rootX] < uf->size[rootY]) {
            uf->parent[rootX] = rootY;
            uf->size[rootY] += uf->size[rootX];
        } else {
            uf->parent[rootY] = rootX;
            uf->size[rootX] += uf->size[rootY];
        }
    }
}

// Function to process union operations and store results
void process_unions(UnionFind *uf, int m, int pairs[][2]) {
    for (int i = 0; i < m; i++) {
        union_nodes(uf, pairs[i][0], pairs[i][1]);
        uf->results[uf->current++] = uf->disconnected_pairs; // Store the current count of disconnected pairs
    }
}

// Function to print results in reverse order
void print_results(UnionFind *uf) {
    for (int i = uf->current - 1; i >= 0; i--) {
        printf("%lld\n", uf->results[i]); // Output results in reverse order
    }
}

// Main function to execute the union-find operations
int main() {
    int n, m;
    scanf("%d %d", &n, &m); // Read number of nodes and operations

    UnionFind *uf = init_union_find(n); // Initialize union-find structure

    int pairs[m][2]; // Array to hold union operation pairs
    for (int i = 0; i < m; i++) {
        scanf("%d %d", &pairs[i][0], &pairs[i][1]); // Read each pair
    }

    process_unions(uf, m, pairs); // Process the union operations
    print_results(uf); // Print results in reverse order

    // Free allocated memory
    free(uf->parent);
    free(uf->size);
    free(uf->results);
    free(uf);
    
    return 0;
}
