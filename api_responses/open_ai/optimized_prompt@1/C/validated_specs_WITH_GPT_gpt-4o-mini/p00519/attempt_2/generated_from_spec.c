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
    if (pq->size < pq->capacity) {
        pq->nodes[pq->size++] = node;
        // Simple insertion sort for maintaining the priority queue
        for (int i = pq->size - 1; i > 0; i--) {
            if (pq->nodes[i].cost < pq->nodes[i - 1].cost) {
                Node temp = pq->nodes[i];
                pq->nodes[i] = pq->nodes[i - 1];
                pq->nodes[i - 1] = temp;
            } else {
                break;
            }
        }
    }
}

Node pop(PriorityQueue *pq) {
    return pq->nodes[--pq->size];
}

int is_empty(PriorityQueue *pq) {
    return pq->size == 0;
}

int main() {
    int N, K;
    if (scanf("%d %d", &N, &K) != 2 || N <= 0 || K < 0) {
        return 1; // Error in input
    }

    int *taxi_costs = malloc(N * sizeof(int));
    int *taxi_limits = malloc(N * sizeof(int));
    if (!taxi_costs || !taxi_limits) {
        free(taxi_costs);
        free(taxi_limits);
        return 1; // Memory allocation failure
    }

    for (int i = 0; i < N; i++) {
        if (scanf("%d %d", &taxi_costs[i], &taxi_limits[i]) != 2) {
            free(taxi_costs);
            free(taxi_limits);
            return 1; // Error in input
        }
    }

    if (K == 0) {
        free(taxi_costs);
        free(taxi_limits);
        return 0; // No connections, terminate without output
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
            free(taxi_costs);
            free(taxi_limits);
            return 1; // Error in input
        }
        connections[a][b] = 1; // Bidirectional connection
        connections[b][a] = 1;
    }

    // Priority queue for pathfinding
    PriorityQueue *pq = create_priority_queue(N * N);
    if (!pq) {
        for (int j = 0; j < N; j++) free(connections[j]);
        free(connections);
        free(taxi_costs);
        free(taxi_limits);
        return 1; // Memory allocation failure
    }

    // Status array to hold minimum costs
    int **status_mincost = malloc(N * sizeof(int *));
    for (int i = 0; i < N; i++) {
        status_mincost[i] = malloc((N + 1) * sizeof(int)); // +1 for remaining rides
        for (int j = 0; j <= N; j++) {
            status_mincost[i][j] = INT_MAX; // Initialize to max cost
        }
    }

    // Start from town 0 with all rides available
    status_mincost[0][taxi_limits[0]] = 0;
    push(pq, (Node){0, 0, taxi_limits[0]});

    while (!is_empty(pq)) {
        Node current = pop(pq);
        int town = current.town;
        int cost = current.cost;
        int remaining_rides = current.remaining_rides;

        // If we reached the last town, print the cost
        if (town == N - 1) {
            printf("%d\n", cost);
            for (int j = 0; j < N; j++) free(connections[j]);
            free(connections);
            free(taxi_costs);
            free(taxi_limits);
            free_priority_queue(pq);
            for (int j = 0; j < N; j++) free(status_mincost[j]);
            free(status_mincost);
            return 0; // Successful termination
        }

        // Explore connections
        for (int next_town = 0; next_town < N; next_town++) {
            if (connections[town][next_town]) {
                int new_cost = cost + taxi_costs[town];
                if (remaining_rides > 0 && new_cost < status_mincost[next_town][remaining_rides - 1]) {
                    status_mincost[next_town][remaining_rides - 1] = new_cost;
                    push(pq, (Node){next_town, new_cost, remaining_rides - 1});
                }
            }
        }
    }

    // If we exit the loop without reaching the last town, terminate without output
    for (int j = 0; j < N; j++) free(connections[j]);
    free(connections);
    free(taxi_costs);
    free(taxi_limits);
    free_priority_queue(pq);
    for (int j = 0; j < N; j++) free(status_mincost[j]);
    free(status_mincost);
    return 0; // Unreachable destination, terminate without output
}
