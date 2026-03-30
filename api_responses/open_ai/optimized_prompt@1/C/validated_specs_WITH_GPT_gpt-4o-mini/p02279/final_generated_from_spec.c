#include <stdio.h>
#include <stdlib.h>

#define MAX_NODES 100000

typedef struct Node {
    int id;                 // Node identifier
    int parent;             // Parent identifier
    int depth;              // Depth of the node
    int left;               // Left child identifier
    int right;              // Right child identifier
} Node;

Node tree[MAX_NODES];       // Array to hold nodes
int nodeCount;              // Number of nodes in the tree

// Function to construct the tree from user input
void constructTree() {
    scanf("%d", &nodeCount); // Read the number of nodes
    for (int i = 0; i < nodeCount; i++) {
        int id, numChildren;
        scanf("%d %d", &id, &numChildren); // Read node id and number of children
        tree[id].id = id; // Set node id
        tree[id].parent = (i == 0) ? -1 : -1; // Root node has no parent
        tree[id].depth = 0; // Initialize depth
        tree[id].left = -1; // Initialize left child
        tree[id].right = -1; // Initialize right child

        // Read children identifiers
        if (numChildren > 0) {
            int childId;
            scanf("%d", &childId); // Read left child
            tree[id].left = childId; // Assign left child
            tree[childId].parent = id; // Set parent of left child
            tree[childId].depth = 1; // Set depth of left child

            Node *current = &tree[id];
            for (int j = 1; j < numChildren; j++) {
                scanf("%d", &childId); // Read right child
                current->right = childId; // Assign right child
                tree[childId].parent = id; // Set parent of right child
                tree[childId].depth = 1; // Set depth of right child
                current = &tree[childId]; // Move to the current child
            }
        }
    }
}

// Function to print the properties of each node
void printNodeProperties() {
    for (int i = 0; i < nodeCount; i++) {
        Node *node = &tree[i];
        // Determine node type
        const char *type;
        if (node->parent == -1) {
            type = "root";
        } else if (node->left == -1 && node->right == -1) {
            type = "leaf";
        } else {
            type = "internal";
        }

        // Print node properties
        printf("Node ID: %d, Parent: %d, Depth: %d, Type: %s, Children: ", 
               node->id, node->parent, node->depth, type);
        
        // Print children identifiers
        if (node->left != -1) {
            printf("%d", node->left);
            int childId = node->left;
            while (tree[childId].right != -1) {
                childId = tree[childId].right;
                printf(" -> %d", childId);
            }
        }
        printf("\n");
    }
}

int main() {
    constructTree(); // Construct the tree from input
    printNodeProperties(); // Print the properties of each node
    return 0;
}
