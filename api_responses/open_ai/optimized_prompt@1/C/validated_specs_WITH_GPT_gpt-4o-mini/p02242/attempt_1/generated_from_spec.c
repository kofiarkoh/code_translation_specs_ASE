#include <stdio.h>
#include <stdlib.h>

#define MAX_NODES 100 // Assuming a maximum number of nodes for simplicity
#define INF -1 // Using -1 to indicate unreachable nodes

typedef struct {
    int destination;
    int cost;
} Edge;

typedef struct {
    int index;
    int edge_count;
    Edge edges[MAX_NODES]; // Assuming a maximum number of edges per node
} Node;

Node graph[MAX_NODES];
int costs[MAX_NODES];
int visited[MAX_NODES];

void initialize_costs(int num_nodes) {
    for (int i = 0; i < num_nodes; i++) {
        costs[i] = INF; // Initialize all costs to -1 (unreachable)
        visited[i] = 0; // Mark all nodes as unvisited
    }
    costs[0] = 0; // Cost of the source node (node 0) is initialized to 0
}

void dijkstra(int num_nodes) {
    for (int i = 0; i < num_nodes; i++) {
        int min_cost = INF;
        int min_index = -1;

        // Find the unvisited node with the smallest cost
        for (int j = 0; j < num_nodes; j++) {
            if (!visited[j] && (costs[j] != INF) && (costs[j] < min_cost)) {
                min_cost = costs[j];
                min_index = j;
            }
        }

        // If no reachable node is found, break
        if (min_index == -1) {
            break;
        }

        visited[min_index] = 1; // Mark the node as visited

        // Update the costs of the neighboring nodes
        for (int j = 0; j < graph[min_index].edge_count; j++) {
            int neighbor = graph[min_index].edges[j].destination;
            int edge_cost = graph[min_index].edges[j].cost;

            // Relaxation step
            if (!visited[neighbor] && costs[min_index] != INF) {
                int new_cost = costs[min_index] + edge_cost;
                if (costs[neighbor] == INF || new_cost < costs[neighbor]) {
                    costs[neighbor] = new_cost; // Update the cost if it's lower
                }
            }
        }
    }
}

int main() {
    int num_nodes;
    scanf("%d", &num_nodes); // Read the number of nodes

    if (num_nodes < 0 || num_nodes > MAX_NODES) {
        // Undefined behavior for invalid number of nodes
        return 1; // Exit with error
    }

    // Read the graph structure
    for (int i = 0; i < num_nodes; i++) {
        int node_index, edge_count;
        scanf("%d %d", &node_index, &edge_count);
        
        if (node_index < 0 || node_index >= num_nodes || edge_count < 0) {
            // Undefined behavior for invalid node index or edge count
            return 1; // Exit with error
        }

        graph[node_index].index = node_index;
        graph[node_index].edge_count = edge_count;

        for (int j = 0; j < edge_count; j++) {
            int dest, cost;
            scanf("%d %d", &dest, &cost);
            if (dest < 0 || dest >= num_nodes) {
                // Undefined behavior for invalid destination node
                return 1; // Exit with error
            }
            graph[node_index].edges[j].destination = dest;
            graph[node_index].edges[j].cost = cost;
        }
    }

    initialize_costs(num_nodes); // Initialize costs
    dijkstra(num_nodes); // Run Dijkstra's algorithm

    // Output the shortest path costs
    for (int i = 0; i < num_nodes; i++) {
        printf("%d %d\n", i, costs[i]); // Print node index and its cost
    }

    return 0; // Successful execution
}
