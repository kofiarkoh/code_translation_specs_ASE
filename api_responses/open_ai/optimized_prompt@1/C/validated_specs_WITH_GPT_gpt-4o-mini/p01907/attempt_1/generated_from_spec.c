#include <stdio.h>
#include <stdlib.h>

typedef struct Node {
    int index;
    int depth;
    struct Node** children;
    int child_count;
} Node;

Node* create_node(int index) {
    Node* node = (Node*)malloc(sizeof(Node));
    node->index = index;
    node->depth = -1; // Unvisited
    node->children = NULL;
    node->child_count = 0;
    return node;
}

void add_edge(Node* nodes[], int parent_index, int child_index) {
    nodes[parent_index]->child_count++;
    nodes[parent_index]->children = (Node**)realloc(nodes[parent_index]->children, 
        nodes[parent_index]->child_count * sizeof(Node*));
    nodes[parent_index]->children[nodes[parent_index]->child_count - 1] = nodes[child_index];
}

void depth_first_search(Node* node, int depth, double* weighted_sum, double probability) {
    node->depth = depth; // Mark as visited and set depth
    *weighted_sum += (depth * probability); // Accumulate weighted sum
    for (int i = 0; i < node->child_count; i++) {
        depth_first_search(node->children[i], depth + 1, weighted_sum, probability);
    }
}

int main() {
    double probability;
    int num_nodes;

    // Read inputs
    scanf("%lf", &probability);
    scanf("%d", &num_nodes);

    Node** nodes = (Node**)malloc(num_nodes * sizeof(Node*));
    for (int i = 0; i < num_nodes; i++) {
        nodes[i] = create_node(i);
    }

    for (int i = 0; i < num_nodes - 1; i++) { // Assuming a tree has num_nodes - 1 edges
        int parent, child, cost;
        scanf("%d %d %d", &parent, &child, &cost);
        add_edge(nodes, parent, child);
    }

    double weighted_sum = 0.0;
    depth_first_search(nodes[0], 0, &weighted_sum, probability); // Start DFS from root node

    // Output the result formatted to ten decimal places
    printf("%.10f\n", weighted_sum);

    // Memory leak: allocated memory for nodes is not freed as per specification
    return 0;
}
