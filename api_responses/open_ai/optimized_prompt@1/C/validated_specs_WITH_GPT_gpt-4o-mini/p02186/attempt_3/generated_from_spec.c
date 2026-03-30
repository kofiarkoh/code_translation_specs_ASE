#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *tree;      // Segment tree array
    int *lazy;      // Lazy propagation array
    int size;       // Size of the segment tree
} SegmentTree;

// Function to create a segment tree
SegmentTree* create_segment_tree(int n) {
    SegmentTree *st = (SegmentTree *)malloc(sizeof(SegmentTree));
    if (!st) {
        exit(EXIT_FAILURE); // Handle memory allocation failure
    }
    st->size = n;
    st->tree = (int *)malloc(4 * n * sizeof(int)); // Allocate memory for segment tree
    st->lazy = (int *)malloc(4 * n * sizeof(int)); // Allocate memory for lazy array
    if (!st->tree || !st->lazy) {
        free(st);
        exit(EXIT_FAILURE); // Handle memory allocation failure
    }
    for (int i = 0; i < 4 * n; i++) {
        st->tree[i] = 0; // Initialize segment tree
        st->lazy[i] = 0; // Initialize lazy array
    }
    return st;
}

// Function to propagate the lazy values
void propagate(SegmentTree *st, int node, int start, int end) {
    if (st->lazy[node] != 0) {
        st->tree[node] += st->lazy[node]; // Apply the lazy value
        if (start != end) { // Not a leaf node
            st->lazy[node * 2 + 1] += st->lazy[node]; // Propagate to left child
            st->lazy[node * 2 + 2] += st->lazy[node]; // Propagate to right child
        }
        st->lazy[node] = 0; // Clear the lazy value
    }
}

// Function to update a range
void update_range(SegmentTree *st, int node, int start, int end, int l, int r, int value) {
    propagate(st, node, start, end); // Ensure all previous updates are applied
    if (start > end || start > r || end < l) {
        return; // Out of range
    }
    if (start >= l && end <= r) {
        st->lazy[node] += value; // Mark the current node as lazy
        propagate(st, node, start, end); // Apply the lazy value
        return;
    }
    int mid = (start + end) / 2;
    update_range(st, node * 2 + 1, start, mid, l, r, value); // Update left child
    update_range(st, node * 2 + 2, mid + 1, end, l, r, value); // Update right child
    st->tree[node] = (st->tree[node * 2 + 1] > st->tree[node * 2 + 2]) ? 
                     st->tree[node * 2 + 1] : st->tree[node * 2 + 2]; // Update current node
}

// Function to query the maximum value in a range
int query_max(SegmentTree *st, int node, int start, int end, int l, int r) {
    propagate(st, node, start, end); // Ensure all previous updates are applied
    if (start > end || start > r || end < l) {
        return -1; // Out of range, return a value that won't affect max
    }
    if (start >= l && end <= r) {
        return st->tree[node]; // Current segment is fully in range
    }
    int mid = (start + end) / 2;
    int left_max = query_max(st, node * 2 + 1, start, mid, l, r); // Query left child
    int right_max = query_max(st, node * 2 + 2, mid + 1, end, l, r); // Query right child
    return (left_max > right_max) ? left_max : right_max; // Return the maximum
}

// Main function to demonstrate the segment tree
int main() {
    int n, q;
    scanf("%d %d", &n, &q); // Read number of elements and queries
    SegmentTree *st = create_segment_tree(n);
    
    // Initialize the segment tree with input values
    for (int i = 0; i < n; i++) {
        int value;
        scanf("%d", &value);
        update_range(st, 0, 0, n - 1, i, i, value); // Initialize each element
    }

    // Output the maximum value after initialization
    printf("%d\n", query_max(st, 0, 0, n - 1, 0, n - 1));

    // Process each query
    for (int j = 0; j < q; j++) {
        int i, t;
        scanf("%d %d", &i, &t); // Read index and new value
        update_range(st, 0, 0, n - 1, i, i, t); // Update the segment tree
        printf("%d\n", query_max(st, 0, 0, n - 1, 0, n - 1)); // Output the maximum value
    }

    // Free allocated memory
    free(st->tree);
    free(st->lazy);
    free(st);
    return 0;
}
