#include <stdio.h>
#include <stdlib.h>

#define MAX_FAMILY_MEMBERS 1000 // Define a maximum limit for family members
#define MODULO 10007 // Modulo value for the output

// Union-Find structure
typedef struct {
    int parent[MAX_FAMILY_MEMBERS];
    int size[MAX_FAMILY_MEMBERS];
    int maxValue[MAX_FAMILY_MEMBERS];
} UnionFind;

// Initialize the Union-Find structure
void initUnionFind(UnionFind *uf, int n) {
    for (int i = 0; i < n; i++) {
        uf->parent[i] = i; // Each member is its own parent
        uf->size[i] = 1; // Initial size of each component is 1
        uf->maxValue[i] = 0; // Initialize max values to 0
    }
}

// Find the root of the component
int find(UnionFind *uf, int x) {
    if (uf->parent[x] != x) {
        uf->parent[x] = find(uf, uf->parent[x]); // Path compression
    }
    return uf->parent[x];
}

// Union two components
void unionComponents(UnionFind *uf, int x, int y, int valueX, int valueY) {
    int rootX = find(uf, x);
    int rootY = find(uf, y);
    
    if (rootX != rootY) {
        // Union by size
        if (uf->size[rootX] < uf->size[rootY]) {
            uf->parent[rootX] = rootY;
            uf->size[rootY] += uf->size[rootX];
            // Update max value for the new root
            uf->maxValue[rootY] = (uf->maxValue[rootY] > valueX) ? uf->maxValue[rootY] : valueX;
        } else {
            uf->parent[rootY] = rootX;
            uf->size[rootX] += uf->size[rootY];
            // Update max value for the new root
            uf->maxValue[rootX] = (uf->maxValue[rootX] > valueY) ? uf->maxValue[rootX] : valueY;
        }
    }
}

// Main function to process input and compute distinct group formations
int main() {
    int n; // Number of family members
    scanf("%d", &n);
    
    if (n <= 0 || n > MAX_FAMILY_MEMBERS) {
        return 0; // Invalid input based on preconditions
    }

    UnionFind uf;
    initUnionFind(&uf, n);

    int x, y, valueX, valueY;
    while (scanf("%d %d %d %d", &x, &y, &valueX, &valueY) == 4) {
        unionComponents(&uf, x, y, valueX, valueY);
    }

    // Count distinct groups
    int distinctGroups = 0;
    for (int i = 0; i < n; i++) {
        if (uf.parent[i] == i) { // If it's a root
            distinctGroups++;
        }
    }

    // Output the result modulo 10007
    printf("%d\n", distinctGroups % MODULO);
    return 0;
}
