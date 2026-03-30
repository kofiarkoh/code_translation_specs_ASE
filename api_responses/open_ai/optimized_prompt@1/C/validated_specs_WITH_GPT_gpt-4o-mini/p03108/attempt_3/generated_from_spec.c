#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *parent; // Array to hold the parent of each node
    int *size;   // Array to hold the size of each component
    long long *results; // Array to hold the results for each operation
    int n;       // Number of nodes
    int m;       // Number of union operations
    int current; // Current index for results
} UnionFind;

// Function to initialize the Union-Find structure
UnionFind* init_union_find(int n) {
    UnionFind *uf = malloc(sizeof(UnionFind));
    uf->parent = malloc(n * sizeof(int));
    uf->size = malloc(n * sizeof(int));
    uf->results = malloc(n * sizeof(long long)); // Max size for results
    uf->n = n;
    uf->m = 0;
    uf->current = 0;

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
        // Calculate the number of pairs that will be connected
        long long pairs_connected = (long long)uf->size[rootX] * uf->size[rootY];
        // Update the results
        uf->results[uf->current] -= pairs_connected;

        // Union by size
        if (uf->size[rootX] < uf->size[rootY]) {
            uf->parent[rootX] = rootY;
            uf->size[rootY] += uf->size[rootX];
        } else {
            uf->parent[rootY] = rootX;
            uf->size[rootX] += uf->size[rootY];
        }
    }
}

// Function to process union operations and calculate disconnected pairs
void process_unions(UnionFind *uf, int m, int pairs[][2]) {
    // Initial count of disconnected pairs
    uf->results[0] = (long long)uf->n * (uf->n - 1) / 2;

    for (int i = 0; i < m; i++) {
        union_nodes(uf, pairs[i][0], pairs[i][1]);
        uf->current++;
        uf->results[uf->current] = uf->results[0]; // Store the current count
    }
}

// Function to print results in reverse order
void print_results(UnionFind *uf) {
    for (int i = uf->current; i >= 0; i--) {
        printf("%lld\n", uf->results[i]);
    }
}

// Main function to read input and execute the union-find operations
int main() {
    int n, m;
    scanf("%d %d", &n, &m);
    
    UnionFind *uf = init_union_find(n);
    
    int pairs[m][2];
    for (int i = 0; i < m; i++) {
        scanf("%d %d", &pairs[i][0], &pairs[i][1]);
    }

    process_unions(uf, m, pairs);
    print_results(uf);

    // Free allocated memory
    free(uf->parent);
    free(uf->size);
    free(uf->results);
    free(uf);
    
    return 0;
}
