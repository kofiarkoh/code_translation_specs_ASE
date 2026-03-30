#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MAX_NODES 100
#define MAX_COST INT_MAX

// Function to compute the minimum cost to traverse the specified sequence of nodes
int compute_min_cost(int N, int M, int edges[MAX_NODES][3], char types[MAX_NODES], int R, int visit_sequence[MAX_NODES]) {
    // Initialize distance matrices for direct and indirect paths
    int dist[MAX_NODES][MAX_NODES];
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            if (i == j) {
                dist[i][j] = 0; // Cost to traverse from a node to itself is zero
            } else {
                dist[i][j] = MAX_COST; // Initialize to maximum value
            }
        }
    }

    // Process edges to fill the distance matrix
    for (int i = 0; i < M; i++) {
        int x = edges[i][0];
        int y = edges[i][1];
        int s = edges[i][2];
        // Update the distance for direct paths
        if (types[i] == 'd') {
            if (s < dist[x][y]) {
                dist[x][y] = s; // Only keep the minimum cost for direct edges
            }
        }
        // Update the distance for indirect paths
        else if (types[i] == 'i') {
            if (s < dist[x][y]) {
                dist[x][y] = s; // Only keep the minimum cost for indirect edges
            }
        }
    }

    // Floyd-Warshall algorithm to compute all pairs shortest paths
    for (int k = 0; k < N; k++) {
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                if (dist[i][k] < MAX_COST && dist[k][j] < MAX_COST) {
                    if (dist[i][j] > dist[i][k] + dist[k][j]) {
                        dist[i][j] = dist[i][k] + dist[k][j];
                    }
                }
            }
        }
    }

    // Calculate the minimum cost to traverse the specified sequence of nodes
    int total_cost = 0;
    for (int i = 0; i < R - 1; i++) {
        int from = visit_sequence[i];
        int to = visit_sequence[i + 1];
        if (dist[from][to] == MAX_COST) {
            return MAX_COST; // No valid path exists
        }
        total_cost += dist[from][to];
    }

    return total_cost; // Return the total minimum cost
}

int main() {
    int N, M;
    while (scanf("%d %d", &N, &M) == 2) {
        int edges[MAX_NODES][3];
        char types[MAX_NODES];
        for (int i = 0; i < M; i++) {
            scanf("%d %d %d %c", &edges[i][0], &edges[i][1], &edges[i][2], &types[i]);
        }
        int R;
        scanf("%d", &R);
        int visit_sequence[MAX_NODES];
        for (int i = 0; i < R; i++) {
            scanf("%d", &visit_sequence[i]);
        }
        int result = compute_min_cost(N, M, edges, types, R, visit_sequence);
        printf("%d\n", result);
    }
    return 0;
}
