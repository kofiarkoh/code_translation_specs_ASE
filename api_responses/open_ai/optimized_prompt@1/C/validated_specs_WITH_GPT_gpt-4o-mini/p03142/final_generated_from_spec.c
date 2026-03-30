#include <stdio.h>
#include <stdlib.h>

#define MAX_NODES 1000 // Assuming a maximum number of nodes for simplicity

// Function to perform topological sort
void topological_sort(int N, int M, int edges[MAX_NODES][2]) {
    int sansyo[MAX_NODES] = {0}; // Array to track incoming edges
    int next_buffer[MAX_NODES];   // Buffer for nodes with no dependencies
    int front = 0, back = 0;      // Pointers for the buffer
    int processed = 0;            // Count of processed nodes

    // Initialize the graph
    for (int i = 0; i < M; i++) {
        int A = edges[i][0];
        int B = edges[i][1];
        sansyo[B]++; // Increment the incoming edge count for node B
    }

    // Populate the initial buffer with nodes having no incoming edges
    for (int i = 0; i < N; i++) {
        if (sansyo[i] == 0) {
            next_buffer[back++] = i; // Add node to buffer
        }
    }

    // Process nodes until the buffer is empty
    while (front < back) {
        int current = next_buffer[front++]; // Get the next node to process
        printf("%d\n", current);            // Output the node

        // Decrease the incoming edge count for all outgoing edges
        for (int i = 0; i < M; i++) {
            if (edges[i][0] == current) {
                int B = edges[i][1];
                sansyo[B]--; // Decrement the incoming edge count
                if (sansyo[B] == 0) {
                    next_buffer[back++] = B; // Add to buffer if no more dependencies
                }
            }
        }
        processed++; // Increment the count of processed nodes
    }

    // Check if all nodes were processed (to ensure no cycles)
    if (processed != N) {
        fprintf(stderr, "Error: Graph contains a cycle or invalid input.\n");
        exit(EXIT_FAILURE);
    }
}

int main() {
    int N, M;
    if (scanf("%d %d", &N, &M) != 2) { // Read N and M
        fprintf(stderr, "Error: Invalid input for N and M.\n");
        return EXIT_FAILURE;
    }

    int edges[MAX_NODES][2]; // Array to store edges

    // Read edges
    for (int i = 0; i < M; i++) {
        if (scanf("%d %d", &edges[i][0], &edges[i][1]) != 2) {
            fprintf(stderr, "Error: Invalid edge input.\n");
            return EXIT_FAILURE;
        }
    }

    topological_sort(N, M, edges); // Perform topological sort

    return EXIT_SUCCESS; // Successful execution
}
