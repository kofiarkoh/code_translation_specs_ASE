#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MAX_NODES 100 // Assuming a maximum number of nodes for simplicity

int main() {
    int N, M;
    scanf("%d %d", &N, &M);

    // Initialize distance matrix
    long long dist[MAX_NODES][MAX_NODES];
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            if (i == j) {
                dist[i][j] = 0; // Distance to self is zero
            } else {
                dist[i][j] = LLONG_MAX; // Initially unreachable
            }
        }
    }

    // Read edges and update the distance matrix
    for (int i = 0; i < M; i++) {
        int u, v;
        long long weight;
        scanf("%d %d %lld", &u, &v, &weight);
        // Update the distance for parallel edges by taking the minimum weight
        if (weight < dist[u][v]) {
            dist[u][v] = weight;
            dist[v][u] = weight; // Assuming undirected graph
        }
    }

    // Floyd-Warshall algorithm to compute shortest paths
    for (int k = 0; k < N; k++) {
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                if (dist[i][k] != LLONG_MAX && dist[k][j] != LLONG_MAX) {
                    if (dist[i][j] > dist[i][k] + dist[k][j]) {
                        dist[i][j] = dist[i][k] + dist[k][j];
                    }
                }
            }
        }
    }

    // Count edges not part of any shortest path
    int count_not_in_shortest_path = 0;
    for (int i = 0; i < M; i++) {
        int u, v;
        long long weight;
        scanf("%d %d %lld", &u, &v, &weight);
        int is_part_of_shortest_path = 0;

        // Check if this edge is part of any shortest path
        for (int a = 0; a < N; a++) {
            for (int b = 0; b < N; b++) {
                if (dist[a][b] == dist[a][u] + weight + dist[v][b] || 
                    dist[a][b] == dist[a][v] + weight + dist[u][b]) {
                    is_part_of_shortest_path = 1;
                    break;
                }
            }
            if (is_part_of_shortest_path) break;
        }

        if (!is_part_of_shortest_path) {
            count_not_in_shortest_path++;
        }
    }

    // Output the count of edges not part of any shortest path
    printf("%d\n", count_not_in_shortest_path);
    return 0;
}
