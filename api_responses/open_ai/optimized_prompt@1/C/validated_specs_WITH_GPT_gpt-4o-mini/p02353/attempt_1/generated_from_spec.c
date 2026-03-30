#include <stdio.h>
#include <stdlib.h>

typedef struct SegmentTree {
    long long *tree; // Array to store segment tree
    long long *lazy; // Array for lazy propagation
    int size; // Size of the segment tree
} SegmentTree;

// Function to create a segment tree
SegmentTree* createSegmentTree(int n) {
    SegmentTree *st = (SegmentTree *)malloc(sizeof(SegmentTree));
    st->size = n;
    st->tree = (long long *)calloc(4 * n, sizeof(long long));
    st->lazy = (long long *)calloc(4 * n, sizeof(long long));
    return st;
}

// Function to apply pending updates
void applyLazy(SegmentTree *st, int node, int start, int end) {
    if (st->lazy[node] != 0) {
        // Apply the pending update
        st->tree[node] += (end - start + 1) * st->lazy[node]; // Update the sum
        if (start != end) {
            // Not a leaf node, propagate the lazy value to children
            st->lazy[node * 2 + 1] += st->lazy[node];
            st->lazy[node * 2 + 2] += st->lazy[node];
        }
        st->lazy[node] = 0; // Clear the lazy value
    }
}

// Function to update a range with a value
void updateRange(SegmentTree *st, int node, int start, int end, int l, int r, long long value) {
    applyLazy(st, node, start, end); // Apply any pending updates

    if (start > end || start > r || end < l) {
        return; // Out of range
    }

    if (start >= l && end <= r) {
        // Current segment is fully in range
        st->tree[node] += (end - start + 1) * value; // Update the sum
        if (start != end) {
            st->lazy[node * 2 + 1] += value; // Mark children for lazy update
            st->lazy[node * 2 + 2] += value;
        }
        return;
    }

    // Partially in range
    int mid = (start + end) / 2;
    updateRange(st, node * 2 + 1, start, mid, l, r, value);
    updateRange(st, node * 2 + 2, mid + 1, end, l, r, value);
    st->tree[node] = st->tree[node * 2 + 1] + st->tree[node * 2 + 2]; // Update current node
}

// Function to get the sum of a range
long long getSum(SegmentTree *st, int node, int start, int end, int l, int r) {
    applyLazy(st, node, start, end); // Apply any pending updates

    if (start > end || start > r || end < l) {
        return 0; // Out of range
    }

    if (start >= l && end <= r) {
        return st->tree[node]; // Current segment is fully in range
    }

    // Partially in range
    int mid = (start + end) / 2;
    long long sumLeft = getSum(st, node * 2 + 1, start, mid, l, r);
    long long sumRight = getSum(st, node * 2 + 2, mid + 1, end, l, r);
    return sumLeft + sumRight; // Return the total sum
}

// Main function to process input and execute commands
int main() {
    int n, q;
    scanf("%d %d", &n, &q); // Read number of elements and queries

    SegmentTree *st = createSegmentTree(n); // Create segment tree

    for (int i = 0; i < q; i++) {
        int command, l, r;
        scanf("%d %d %d", &command, &l, &r); // Read command and range
        if (command == 1) { // Update command
            long long value;
            scanf("%lld", &value); // Read value to update
            updateRange(st, 0, 0, n - 1, l, r, value); // Update range
        } else if (command == 2) { // Get sum command
            long long result = getSum(st, 0, 0, n - 1, l, r); // Get sum
            printf("%lld\n", result); // Output the result
        }
    }

    // Free allocated memory
    free(st->tree);
    free(st->lazy);
    free(st);
    return 0;
}
