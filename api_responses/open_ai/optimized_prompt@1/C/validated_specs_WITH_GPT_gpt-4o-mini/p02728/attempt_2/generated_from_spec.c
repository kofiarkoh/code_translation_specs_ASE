#include <stdio.h>
#include <stdlib.h>

#define MODULO_CONSTANT 1000000007 // Define a constant for modulo operation

// Function to perform depth-first search and compute values for each node
void dfs(int node, int parent, int *a, int *b, long long *tans, int *tS, int n) {
    tans[node] = 1; // Initialize the computed value for the current node to 1
    tS[node] = 0;   // Initialize the count of child nodes

    for (int i = 0; i < n - 1; i++) {
        // Check if the current edge connects to the current node
        if (a[i] == node + 1 && b[i] != parent) { // a[i] is 1-based, convert to 0-based
            dfs(b[i] - 1, node, a, b, tans, tS, n); // Recursive DFS call for child
            tans[node] = (tans[node] * tans[b[i] - 1]) % MODULO_CONSTANT; // Update tans
            tS[node]++; // Increment child count
        } else if (b[i] == node + 1 && a[i] != parent) { // Check the reverse edge
            dfs(a[i] - 1, node, a, b, tans, tS, n); // Recursive DFS call for child
            tans[node] = (tans[node] * tans[a[i] - 1]) % MODULO_CONSTANT; // Update tans
            tS[node]++; // Increment child count
        }
    }
}

long long* compute_tree_values(int n, int *a, int *b) {
    long long *tans = (long long *)malloc(n * sizeof(long long)); // Output array for computed values
    int *tS = (int *)malloc(n * sizeof(int)); // Array to count child nodes

    // Start DFS from the root node (assumed to be node 0)
    dfs(0, -1, a, b, tans, tS, n);

    free(tS); // Free the child count array
    return tans; // Return the computed values
}

int main() {
    int n; // Number of nodes
    scanf("%d", &n); // Read number of nodes

    int *a = (int *)malloc((n - 1) * sizeof(int)); // Array for edges
    int *b = (int *)malloc((n - 1) * sizeof(int)); // Array for edges

    // Read edges
    for (int i = 0; i < n - 1; i++) {
        scanf("%d %d", &a[i], &b[i]); // Read each edge
    }

    long long *result = compute_tree_values(n, a, b); // Compute values

    // Output the results
    for (int i = 0; i < n; i++) {
        printf("%lld\n", result[i]); // Print each computed value
    }

    free(a); // Free the edges array
    free(b); // Free the edges array
    free(result); // Free the result array

    return 0; // Exit the program
}
