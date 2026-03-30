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
    if (!pq) return NULL; // Error handling for memory allocation
    pq->nodes = malloc(capacity * sizeof(Node));
    if (!pq->nodes) {
        free(pq);
        return NULL; // Error handling for memory allocation
    }
    pq->size = 0;
    pq->capacity = capacity;
    return pq;
}

void free_priority_queue(PriorityQueue *pq) {
    free(pq->nodes);
    free(pq);
}

void swap(Node *a, Node *b) {
    Node temp = *a;
    *a = *b;
    *b = temp;
}

void push(PriorityQueue *pq, Node node) {
    if (pq->size >= pq->capacity) return; // Prevent overflow
    pq->nodes[pq->size++] = node;
    // Bubble up
    for (int i = pq->size - 1; i > 0; i = (i - 1) / 2) {
        if (pq->nodes[i].cost < pq->nodes[(i - 1) / 2].cost) {
            swap(&pq->nodes[i], &pq->nodes[(i - 1) / 2]);
        } else {
            break;
        }
    }
}

Node pop(PriorityQueue *pq) {
    Node min = pq->nodes[0];
    pq->nodes[0] = pq->nodes[--pq->size];
    // Bubble down
    for (int i = 0; 2 * i + 1 < pq->size;) {
        int child = 2 * i + 1;
        if (child + 1 < pq->size && pq->nodes[child + 1].cost < pq->nodes[child].cost) {
            child++;
        }
        if (pq->nodes[i].cost > pq->nodes[child].cost) {
            swap(&pq->nodes[i], &pq->nodes[child]);
            i = child;
        } else {
            break;
        }
    }
    return min;
}

int main() {
    int N, K;
    scanf("%d %d", &N, &K);
    
    if (N <= 0 || K < 0) return 1; // Invalid input handling

    int *taxi_costs = malloc(N * sizeof(int));
    int *taxi_limits = malloc(N * sizeof(int));
    if (!taxi_costs || !taxi_limits) return 1; // Memory allocation error

    for (int i = 0; i < N; i++) {
        scanf("%d %d", &taxi_costs[i], &taxi_limits[i]);
        if (taxi_costs[i] < 0 || taxi_limits[i] < 0) {
            free(taxi_costs);
            free(taxi_limits);
            return 1; // Invalid input handling
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
        scanf("%d %d", &a, &b);
        connections[a][b] = 1; // Bidirectional connection
        connections[b][a] = 1;
    }

    if (K == 0) {
        free(taxi_costs);
        free(taxi_limits);
        for (int i = 0; i < N; i++) free(connections[i]);
        free(connections);
        return 0; // No connections, terminate without output
    }

    int *status_mincost = malloc(N * sizeof(int));
    for (int i = 0; i < N; i++) {
        status_mincost[i] = INT_MAX; // Initialize costs to infinity
    }
    status_mincost[0] = 0; // Starting point cost

    PriorityQueue *pq = create_priority_queue(N);
    if (!pq) {
        free(taxi_costs);
        free(taxi_limits);
        for (int i = 0; i < N; i++) free(connections[i]);
        free(connections);
        return 1; // Memory allocation error
    }

    push(pq, (Node){0, 0, taxi_limits[0]}); // Start from town 0

    while (pq->size > 0) {
        Node current = pop(pq);
        int town = current.town;

        if (town == N - 1) {
            printf("%d\n", current.cost); // Output minimum cost to reach last town
            free_priority_queue(pq);
            free(taxi_costs);
            free(taxi_limits);
            for (int i = 0; i < N; i++) free(connections[i]);
            free(connections);
            free(status_mincost);
            return 0; // Successfully reached destination
        }

        for (int next_town = 0; next_town < N; next_town++) {
            if (connections[town][next_town]) {
                int new_cost = current.cost + taxi_costs[next_town];
                if (new_cost < status_mincost[next_town]) {
                    status_mincost[next_town] = new_cost;
                    push(pq, (Node){next_town, new_cost, taxi_limits[next_town]});
                }
            }
        }
    }

    // If we exit the loop without reaching the last town, do nothing (terminate without output)
    free_priority_queue(pq);
    free(taxi_costs);
    free(taxi_limits);
    for (int i = 0; i < N; i++) free(connections[i]);
    free(connections);
    free(status_mincost);
    return 0;
}
