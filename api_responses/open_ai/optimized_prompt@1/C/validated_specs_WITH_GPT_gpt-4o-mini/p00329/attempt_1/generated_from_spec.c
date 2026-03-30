#include <stdio.h>
#include <stdlib.h>

#define MAX_N 1005

int grid[MAX_N][MAX_N]; // Adjacency matrix for connections
int match[MAX_N];       // Array to store matches for nodes in the second set
int seen[MAX_N];        // Array to track seen nodes during DFS

// Function to perform DFS for finding an augmenting path
int bpm(int u, int N) {
    for (int v = 0; v < N; v++) {
        // If there is a connection and v is not seen
        if (grid[u][v] && !seen[v]) {
            seen[v] = 1; // Mark v as seen
            // If v is not matched or previously matched node can find an alternate match
            if (match[v] == -1 || bpm(match[v], N)) {
                match[v] = u; // Match v with u
                return 1; // Found a match
            }
        }
    }
    return 0; // No match found
}

// Function to find if a perfect matching exists
int isPerfectMatching(int N) {
    // Initialize matches to -1 (no matches)
    for (int i = 0; i < N; i++) {
        match[i] = -1;
    }

    int result = 0; // Count of matches
    for (int u = 0; u < N; u++) {
        // Reset seen array for each node in the first set
        for (int i = 0; i < N; i++) {
            seen[i] = 0;
        }
        // Try to find a match for u
        if (bpm(u, N)) {
            result++; // Increment match count
        }
    }
    return result; // Return the total number of matches found
}

int main() {
    int N;
    // Read the number of nodes
    scanf("%d", &N);

    // Read the grid of connections
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            scanf("%d", &grid[i][j]);
        }
    }

    // Check for perfect matching
    int matches = isPerfectMatching(N);
    if (matches == N) {
        printf("yes\n");
        // Output the matched pairs with an offset of +1
        for (int i = 0; i < N; i++) {
            printf("%d ", match[i] + 1); // Adjusting index for output
        }
        printf("\n");
    } else {
        printf("no\n");
    }

    return 0;
}
