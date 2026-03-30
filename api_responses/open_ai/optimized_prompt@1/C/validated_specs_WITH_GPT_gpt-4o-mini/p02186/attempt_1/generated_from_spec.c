#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

typedef struct {
    int *tree;       // Segment tree array
    int *lazy;       // Lazy propagation array
    int size;        // Size of the segment tree
} SegmentTree;

// Function to create a segment tree
SegmentTree* create_segment_tree(int n) {
    SegmentTree *st = (SegmentTree *)malloc(sizeof(SegmentTree));
    if (!st) {
        exit(EXIT_FAILURE); // Terminate if memory allocation fails
    }
    st->size = n;
    st->tree = (int *)malloc(4 * n * sizeof(int)); // Allocate memory for segment tree
    st->lazy = (int *)malloc(4 * n * sizeof(int)); // Allocate memory for lazy array
    if (!st->tree || !st->lazy) {
        free(st);
        exit(EXIT_FAILURE); // Terminate if memory allocation fails
    }
    for (int i = 0; i < 4 * n; i++) {
        st->tree[i] = INT_MIN; // Initialize segment tree with minimum values
        st->lazy[i] = 0;       // Initialize lazy array with zeros
    }
    return st;
}

// Function to update the segment tree lazily
void update_range(SegmentTree *st, int node, int start, int end, int idx, int value) {
    if (start == end) {
        st->tree[node] = value; // Update the leaf node
    } else {
        int mid = (start + end) / 2;
        if (idx <= mid) {
            update_range(st, 2 * node + 1, start, mid, idx, value); // Update left child
        } else {
            update_range(st, 2 * node + 2, mid + 1, end, idx, value); // Update right child
        }
        st->tree[node] = (st->tree[2 * node + 1] > st->tree[2 * node + 2]) ? 
                          st->tree[2 * node + 1] : st->tree[2 * node + 2]; // Update current node
    }
}

// Function to get the maximum value in the range
int query_max(SegmentTree *st, int node, int start, int end) {
    if (start == end) {
        return st->tree[node]; // Return the leaf node value
    }
    int mid = (start + end) / 2;
    int left_max = query_max(st, 2 * node + 1, start, mid); // Query left child
    int right_max = query_max(st, 2 * node + 2, mid + 1, end); // Query right child
    return (left_max > right_max) ? left_max : right_max; // Return the maximum value
}

// Main function to handle input and output
int main() {
    int n, q;
    scanf("%d %d", &n, &q); // Read number of elements and queries
    int *arr = (int *)malloc(n * sizeof(int)); // Allocate memory for input array
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]); // Read the input sequence
    }

    SegmentTree *st = create_segment_tree(n); // Create the segment tree

    // Initialize the segment tree with the input array
    for (int i = 0; i < n; i++) {
        update_range(st, 0, 0, n - 1, i, arr[i]); // Update segment tree with initial values
    }

    // Output the maximum value after initialization
    printf("%d\n", query_max(st, 0, 0, n - 1));

    // Process each query
    for (int j = 0; j < q; j++) {
        int i, t;
        scanf("%d %d", &i, &t); // Read index and new value
        if (i >= 0 && i < n) { // Check bounds
            update_range(st, 0, 0, n - 1, i, t); // Update the segment tree
            printf("%d\n", query_max(st, 0, 0, n - 1)); // Output the maximum value after the update
        }
    }

    // Free allocated memory
    free(arr);
    free(st->tree);
    free(st->lazy);
    free(st);
    return 0;
}
