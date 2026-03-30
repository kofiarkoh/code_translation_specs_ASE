#include <stdio.h>
#include <stdlib.h>

#define MAX_N 1005

int grid[MAX_N][MAX_N]; // Adjacency matrix for connections
int match[MAX_N];       // Array to store matches for nodes in the second set
int seen[MAX_N];        // Array to track seen nodes during DFS
int N;                  // Number of nodes in each set

// Depth-first search to find an augmenting path
int bpm(int u) {
    for (int v = 0; v < N; v++) {
        // If there is a connection and v is not seen
        if (grid[u][v] && !seen[v]) {
            seen[v] = 1; // Mark v as seen
            // If v is not matched or previously matched node can find an alternate match
            if (match[v] == -1 || bpm(match[v])) {
                match[v] = u; // Match v with u
                return 1;     // Found a match
            }
        }
    }
    return 0; // No match found
}

int main() {
    // Read input
    scanf("%d", &N);
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            scanf("%d", &grid[i][j]);
        }
    }

    // Initialize matches to -1 (no matches)
    for (int i = 0; i < N; i++) {
        match[i] = -1;
    }

    int result = 0; // Count of matches found
    for (int u = 0; u < N; u++) {
        // Reset seen array for each node in the first set
        for (int i = 0; i < N; i++) {
            seen[i] = 0;
        }
        // Try to find a match for u
        if (bpm(u)) {
            result++;
        }
    }

    // Output result
    if (result == N) {
        printf("yes\n");
        for (int i = 0; i < N; i++) {
            // Output matched nodes from the second set, adjusted by +1 for 1-based index
            printf("%d ", match[i] + 1);
        }
        printf("\n");
    } else {
        printf("no\n");
    }

    return 0;
}
