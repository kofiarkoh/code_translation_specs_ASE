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

void initialize_costs(int num_nodes, int source) {
    for (int i = 0; i < num_nodes; i++) {
        costs[i] = INF; // Initialize all costs to -1 (unreachable)
        visited[i] = 0; // Mark all nodes as unvisited
    }
    costs[source] = 0; // Cost to reach the source node is 0
}

void dijkstra(int num_nodes, int source) {
    for (int i = 0; i < num_nodes; i++) {
        int min_cost = INF;
        int min_index = -1;

        // Find the unvisited node with the smallest cost
        for (int j = 0; j < num_nodes; j++) {
            if (!visited[j] && (costs[j] != INF) && (costs[j] < min_cost || min_cost == INF)) {
                min_cost = costs[j];
                min_index = j;
            }
        }

        // If no reachable node is found, break
        if (min_index == -1) {
            break;
        }

        visited[min_index] = 1; // Mark the node as visited

        // Update the costs for the neighbors of the current node
        for (int j = 0; j < graph[min_index].edge_count; j++) {
            Edge edge = graph[min_index].edges[j];
            int new_cost = costs[min_index] + edge.cost;

            // Only update if the new cost is less than the current cost
            if (costs[edge.destination] == INF || new_cost < costs[edge.destination]) {
                costs[edge.destination] = new_cost;
            }
        }
    }
}

int main() {
    int num_nodes, source;

    // Read the number of nodes
    scanf("%d", &num_nodes);
    if (num_nodes <= 0 || num_nodes > MAX_NODES) {
        return 1; // Undefined behavior for invalid input
    }

    // Read the graph structure
    for (int i = 0; i < num_nodes; i++) {
        int node_index, edge_count;
        scanf("%d %d", &node_index, &edge_count);
        graph[node_index].index = node_index;
        graph[node_index].edge_count = edge_count;

        for (int j = 0; j < edge_count; j++) {
            int dest, cost;
            scanf("%d %d", &dest, &cost);
            graph[node_index].edges[j].destination = dest;
            graph[node_index].edges[j].cost = cost;
        }
    }

    // Read the source node
    scanf("%d", &source);
    if (source < 0 || source >= num_nodes) {
        return 1; // Undefined behavior for invalid input
    }

    initialize_costs(num_nodes, source);
    dijkstra(num_nodes, source);

    // Output the shortest path costs
    for (int i = 0; i < num_nodes; i++) {
        printf("%d %d\n", i, costs[i]);
    }

    return 0;
}
