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

// Function to propagate updates in the segment tree
void propagate(SegmentTree *st, int node, int start, int end) {
    if (st->lazy[node] != 0) {
        st->tree[node] = st->lazy[node]; // Apply the lazy value
        if (start != end) { // Not a leaf node
            st->lazy[node * 2 + 1] = st->lazy[node]; // Mark children for lazy update
            st->lazy[node * 2 + 2] = st->lazy[node];
        }
        st->lazy[node] = 0; // Clear lazy value
    }
}

// Function to update a value in the segment tree
void update(SegmentTree *st, int idx, int value, int node, int start, int end) {
    propagate(st, node, start, end); // Ensure any pending updates are applied
    if (start == end) { // Leaf node
        st->tree[node] = value; // Update value
    } else {
        int mid = (start + end) / 2;
        if (idx <= mid) {
            update(st, idx, value, node * 2 + 1, start, mid); // Update left child
        } else {
            update(st, idx, value, node * 2 + 2, mid + 1, end); // Update right child
        }
        st->tree[node] = (st->tree[node * 2 + 1] > st->tree[node * 2 + 2]) ? 
                          st->tree[node * 2 + 1] : st->tree[node * 2 + 2]; // Update current node
    }
}

// Function to query the maximum value in the segment tree
int query(SegmentTree *st, int l, int r, int node, int start, int end) {
    propagate(st, node, start, end); // Ensure any pending updates are applied
    if (r < start || end < l) {
        return -1; // Out of range
    }
    if (l <= start && end <= r) {
        return st->tree[node]; // Current segment is fully in range
    }
    int mid = (start + end) / 2;
    int left_max = query(st, l, r, node * 2 + 1, start, mid);
    int right_max = query(st, l, r, node * 2 + 2, mid + 1, end);
    return (left_max > right_max) ? left_max : right_max; // Return maximum of both sides
}

// Main function to handle input and output
int main() {
    int n, q;
    scanf("%d %d", &n, &q); // Read number of elements and queries
    SegmentTree *st = create_segment_tree(n);
    
    // Read initial values
    for (int i = 0; i < n; i++) {
        int value;
        scanf("%d", &value);
        update(st, i, value, 0, 0, n - 1); // Initialize segment tree
    }

    // Output the maximum value after initialization
    printf("%d\n", query(st, 0, n - 1, 0, 0, n - 1));

    // Process each query
    for (int i = 0; i < q; i++) {
        int idx, value;
        scanf("%d %d", &idx, &value); // Read index and new value
        update(st, idx, value, 0, 0, n - 1); // Update segment tree
        printf("%d\n", query(st, 0, n - 1, 0, 0, n - 1)); // Output maximum value
    }

    // Free allocated memory
    free(st->tree);
    free(st->lazy);
    free(st);
    return 0;
}
