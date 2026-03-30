#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007

typedef struct Node {
    int id;
    struct Node** children;
    int child_count;
    long subtree_size;
} Node;

Node* create_node(int id) {
    Node* node = (Node*)malloc(sizeof(Node));
    node->id = id;
    node->children = NULL;
    node->child_count = 0;
    node->subtree_size = 0;
    return node;
}

void add_child(Node* parent, Node* child) {
    parent->children = (Node**)realloc(parent->children, sizeof(Node*) * (parent->child_count + 1));
    parent->children[parent->child_count] = child;
    parent->child_count++;
}

void dfs(Node* node) {
    node->subtree_size = 1; // Count the node itself
    for (int i = 0; i < node->child_count; i++) {
        dfs(node->children[i]);
        node->subtree_size += node->children[i]->subtree_size; // Accumulate subtree sizes
    }
}

long compute_result(Node* root) {
    long result = 1; // Initialize result for combinatorial calculation
    // Example combinatorial calculation based on subtree sizes
    for (int i = 0; i < root->child_count; i++) {
        result = (result * root->children[i]->subtree_size) % MOD; // Example operation
    }
    return result;
}

int main() {
    int n;
    scanf("%d", &n); // Read number of nodes

    if (n < 1) return 0; // Precondition check

    Node** nodes = (Node**)malloc(sizeof(Node*) * n);
    for (int i = 0; i < n; i++) {
        nodes[i] = create_node(i + 1); // Create nodes
    }

    for (int i = 0; i < n - 1; i++) {
        int u, v;
        scanf("%d %d", &u, &v); // Read edges
        add_child(nodes[u - 1], nodes[v - 1]); // Assuming 1-based index for input
        add_child(nodes[v - 1], nodes[u - 1]); // Add both directions for undirected tree
    }

    dfs(nodes[0]); // Start DFS from the root node (assumed to be node 1)

    long result = compute_result(nodes[0]); // Compute the result based on the tree
    printf("%ld\n", (result + MOD) % MOD); // Ensure non-negative output

    // Free allocated memory
    for (int i = 0; i < n; i++) {
        free(nodes[i]->children);
        free(nodes[i]);
    }
    free(nodes);

    return 0;
}
