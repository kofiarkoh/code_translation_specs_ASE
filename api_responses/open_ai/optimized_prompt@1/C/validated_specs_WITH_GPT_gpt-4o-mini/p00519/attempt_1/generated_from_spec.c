#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

typedef struct {
    int town;
    int cost;
    int remaining_rides;
} Node;

typedef struct {
    Node *nodes;
    int size;
    int capacity;
} PriorityQueue;

PriorityQueue* create_priority_queue(int capacity) {
    PriorityQueue *pq = malloc(sizeof(PriorityQueue));
    if (!pq) return NULL;
    pq->nodes = malloc(capacity * sizeof(Node));
    if (!pq->nodes) {
        free(pq);
        return NULL;
    }
    pq->size = 0;
    pq->capacity = capacity;
    return pq;
}

void free_priority_queue(PriorityQueue *pq) {
    free(pq->nodes);
    free(pq);
}

void push(PriorityQueue *pq, Node node) {
    // Insert node into the priority queue (min-heap)
    pq->nodes[pq->size++] = node;
    // Simple bubble-up to maintain heap property
    int i = pq->size - 1;
    while (i > 0) {
        int parent = (i - 1) / 2;
        if (pq->nodes[i].cost >= pq->nodes[parent].cost) break;
        Node temp = pq->nodes[i];
        pq->nodes[i] = pq->nodes[parent];
        pq->nodes[parent] = temp;
        i = parent;
    }
}

Node pop(PriorityQueue *pq) {
    // Remove and return the node with the minimum cost
    Node min_node = pq->nodes[0];
    pq->nodes[0] = pq->nodes[--pq->size];
    // Simple bubble-down to maintain heap property
    int i = 0;
    while (1) {
        int left = 2 * i + 1;
        int right = 2 * i + 2;
        int smallest = i;
        if (left < pq->size && pq->nodes[left].cost < pq->nodes[smallest].cost) smallest = left;
        if (right < pq->size && pq->nodes[right].cost < pq->nodes[smallest].cost) smallest = right;
        if (smallest == i) break;
        Node temp = pq->nodes[i];
        pq->nodes[i] = pq->nodes[smallest];
        pq->nodes[smallest] = temp;
        i = smallest;
    }
    return min_node;
}

int main() {
    int N, K;
    if (scanf("%d %d", &N, &K) != 2 || N <= 0 || K < 0) {
        return 1; // Error code for invalid input
    }

    int *taxi_cost = malloc(N * sizeof(int));
    int *taxi_limit = malloc(N * sizeof(int));
    if (!taxi_cost || !taxi_limit) {
        free(taxi_cost);
        free(taxi_limit);
        return 1; // Error code for memory allocation failure
    }

    for (int i = 0; i < N; i++) {
        if (scanf("%d %d", &taxi_cost[i], &taxi_limit[i]) != 2) {
            free(taxi_cost);
            free(taxi_limit);
            return 1; // Error code for invalid input
        }
    }

    int **connections = malloc(N * sizeof(int *));
    for (int i = 0; i < N; i++) {
        connections[i] = malloc(N * sizeof(int));
        for (int j = 0; j < N; j++) {
            connections[i][j] = 0; // Initialize connections
        }
    }

    for (int i = 0; i < K; i++) {
        int a, b;
        if (scanf("%d %d", &a, &b) != 2 || a < 0 || a >= N || b < 0 || b >= N) {
            for (int j = 0; j < N; j++) free(connections[j]);
            free(connections);
            free(taxi_cost);
            free(taxi_limit);
            return 1; // Error code for invalid input
        }
        connections[a][b] = 1; // Bidirectional connection
        connections[b][a] = 1;
    }

    // Dijkstra-like algorithm to find minimum cost
    int *status_mincost = malloc(N * sizeof(int));
    for (int i = 0; i < N; i++) {
        status_mincost[i] = INT_MAX; // Initialize costs to infinity
    }
    status_mincost[0] = 0; // Starting town cost is 0

    PriorityQueue *pq = create_priority_queue(N);
    if (!pq) {
        for (int j = 0; j < N; j++) free(connections[j]);
        free(connections);
        free(taxi_cost);
        free(taxi_limit);
        free(status_mincost);
        return 1; // Error code for memory allocation failure
    }

    push(pq, (Node){0, 0, taxi_limit[0]}); // Start from town 0

    while (pq->size > 0) {
        Node current = pop(pq);
        int town = current.town;
        int cost = current.cost;
        int remaining_rides = current.remaining_rides;

        if (town == N - 1) {
            printf("%d\n", cost); // Output minimum cost to reach last town
            break;
        }

        for (int next_town = 0; next_town < N; next_town++) {
            if (connections[town][next_town]) {
                int new_cost = cost + taxi_cost[next_town];
                if (remaining_rides > 0 && new_cost < status_mincost[next_town]) {
                    status_mincost[next_town] = new_cost;
                    push(pq, (Node){next_town, new_cost, remaining_rides - 1});
                }
            }
        }
    }

    // Clean up
    for (int j = 0; j < N; j++) free(connections[j]);
    free(connections);
    free(taxi_cost);
    free(taxi_limit);
    free(status_mincost);
    free_priority_queue(pq);

    return 0; // Successful termination
}
