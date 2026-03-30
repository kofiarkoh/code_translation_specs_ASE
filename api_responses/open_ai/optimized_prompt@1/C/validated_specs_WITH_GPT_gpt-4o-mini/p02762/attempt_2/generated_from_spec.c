#include <stdio.h>
#include <stdlib.h>

#define N_MAX 1000 // Assuming a maximum size for n

typedef struct {
    int parent; // Parent index
    int size;   // Size of the set (stored as negative for root)
    int removals; // Count of removals for the element
} Element;

Element elements[N_MAX + 1]; // Array to hold elements, 1-indexed

// Find function with path compression
int find(int x) {
    if (elements[x].parent != x) {
        elements[x].parent = find(elements[x].parent);
    }
    return elements[x].parent;
}

// Union function with union by size
void union_sets(int x, int y) {
    int rootX = find(x);
    int rootY = find(y);
    
    if (rootX != rootY) {
        // Increment removals for both elements
        elements[rootX].removals++;
        elements[rootY].removals++;
        
        // Union by size
        if (elements[rootX].size < elements[rootY].size) {
            elements[rootX].parent = rootY;
            elements[rootY].size += elements[rootX].size; // Update size
        } else {
            elements[rootY].parent = rootX;
            elements[rootX].size += elements[rootY].size; // Update size
        }
    }
}

// Initialize the union-find structure
void initialize(int n) {
    for (int i = 1; i <= n; i++) {
        elements[i].parent = i; // Each element is its own parent
        elements[i].size = -1;   // Size initialized to -1 (1 element)
        elements[i].removals = 0; // No removals initially
    }
}

// Process union and query operations
void process_operations(int n, int m, int k, int unions[][2], int queries[][2]) {
    for (int i = 0; i < m; i++) {
        union_sets(unions[i][0], unions[i][1]);
    }
    
    for (int i = 0; i < k; i++) {
        int x = queries[i][0];
        int y = queries[i][1];
        if (find(x) == find(y)) {
            // Increment removals for both elements in the same set
            elements[find(x)].removals++;
            elements[find(y)].removals++;
        }
    }
}

// Output the adjusted sizes of each set
void output_sizes(int n) {
    for (int i = 1; i <= n; i++) {
        int root = find(i);
        // Adjust size by removals
        int adjusted_size = -elements[root].size - elements[root].removals;
        printf("%d\n", adjusted_size);
    }
}

int main() {
    int n, m, k;
    scanf("%d %d %d", &n, &m, &k);
    
    // Initialize union-find structure
    initialize(n);
    
    int unions[m][2];
    for (int i = 0; i < m; i++) {
        scanf("%d %d", &unions[i][0], &unions[i][1]);
    }
    
    int queries[k][2];
    for (int i = 0; i < k; i++) {
        scanf("%d %d", &queries[i][0], &queries[i][1]);
    }
    
    // Process the operations
    process_operations(n, m, k, unions, queries);
    
    // Output the sizes
    output_sizes(n);
    
    return 0;
}
