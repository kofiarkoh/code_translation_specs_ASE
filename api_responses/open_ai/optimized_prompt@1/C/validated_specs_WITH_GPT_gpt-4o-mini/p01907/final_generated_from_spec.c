#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef struct Edge {
    int to;
    int cost;
    struct Edge* next;
} Edge;

typedef struct Node {
    Edge* edges;
    int visited;
    int depth;
} Node;

void dfs(Node* nodes, int current, int depth, float probability, double* weighted_sum) {
    nodes[current].visited = 1; // Mark the node as visited
    nodes[current].depth = depth; // Set the depth of the current node

    // Traverse all edges from the current node
    for (Edge* edge = nodes[current].edges; edge != NULL; edge = edge->next) {
        if (!nodes[edge->to].visited) { // If the connected node is not visited
            dfs(nodes, edge->to, depth + 1, probability, weighted_sum);
        }
    }

    // Calculate the contribution of the current node's edges to the weighted sum
    for (Edge* edge = nodes[current].edges; edge != NULL; edge = edge->next) {
        *weighted_sum += edge->cost * pow(probability, nodes[edge->to].depth);
    }
}

int main() {
    float probability;
    int num_nodes;

    // Read the probability factor and number of nodes
    scanf("%f", &probability);
    scanf("%d", &num_nodes);

    // Allocate memory for nodes
    Node* nodes = (Node*)malloc(num_nodes * sizeof(Node));
    for (int i = 0; i < num_nodes; i++) {
        nodes[i].edges = NULL;
        nodes[i].visited = 0;
        nodes[i].depth = 0;
    }

    // Read edges and construct the tree
    for (int i = 0; i < num_nodes - 1; i++) { // Assuming a tree has num_nodes - 1 edges
        int from, to, cost;
        scanf("%d %d %d", &from, &to, &cost);

        // Create a new edge from 'from' to 'to'
        Edge* new_edge = (Edge*)malloc(sizeof(Edge));
        new_edge->to = to;
        new_edge->cost = cost;
        new_edge->next = nodes[from].edges;
        nodes[from].edges = new_edge;

        // Create a new edge from 'to' to 'from' (undirected graph)
        new_edge = (Edge*)malloc(sizeof(Edge));
        new_edge->to = from;
        new_edge->cost = cost;
        new_edge->next = nodes[to].edges;
        nodes[to].edges = new_edge;
    }

    double weighted_sum = 0.0;

    // Start DFS from the first node (assuming node indices start from 0)
    dfs(nodes, 0, 0, probability, &weighted_sum);

    // Print the result formatted to ten decimal places
    printf("%.10f\n", weighted_sum);

    // Free allocated memory for edges
    for (int i = 0; i < num_nodes; i++) {
        Edge* edge = nodes[i].edges;
        while (edge != NULL) {
            Edge* temp = edge;
            edge = edge->next;
            free(temp);
        }
    }
    free(nodes); // Free the nodes array

    return 0;
}
