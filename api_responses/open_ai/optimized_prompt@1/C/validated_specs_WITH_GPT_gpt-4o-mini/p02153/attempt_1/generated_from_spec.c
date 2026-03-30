#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <string.h>

#define MAX_SIZE 1000 // Assuming maximum grid size for height and width
#define INF "INF"

typedef struct {
    int x, y;
    long cost;
} Node;

typedef struct {
    Node *nodes;
    int size;
    int capacity;
} PriorityQueue;

void initQueue(PriorityQueue *pq, int capacity) {
    pq->nodes = (Node *)malloc(capacity * sizeof(Node));
    pq->size = 0;
    pq->capacity = capacity;
}

void freeQueue(PriorityQueue *pq) {
    free(pq->nodes);
}

void push(PriorityQueue *pq, Node node) {
    if (pq->size >= pq->capacity) return; // Prevent overflow
    pq->nodes[pq->size++] = node; // Add node
    // Simple bubble up to maintain min-heap property
    for (int i = pq->size - 1; i > 0; i = (i - 1) / 2) {
        if (pq->nodes[i].cost < pq->nodes[(i - 1) / 2].cost) {
            Node temp = pq->nodes[i];
            pq->nodes[i] = pq->nodes[(i - 1) / 2];
            pq->nodes[(i - 1) / 2] = temp;
        } else {
            break;
        }
    }
}

Node pop(PriorityQueue *pq) {
    Node minNode = pq->nodes[0];
    pq->nodes[0] = pq->nodes[--pq->size]; // Replace root with last element
    // Simple bubble down to maintain min-heap property
    for (int i = 0; i * 2 + 1 < pq->size;) {
        int child = i * 2 + 1;
        if (child + 1 < pq->size && pq->nodes[child + 1].cost < pq->nodes[child].cost) {
            child++;
        }
        if (pq->nodes[i].cost > pq->nodes[child].cost) {
            Node temp = pq->nodes[i];
            pq->nodes[i] = pq->nodes[child];
            pq->nodes[child] = temp;
            i = child;
        } else {
            break;
        }
    }
    return minNode;
}

int isValid(int x, int y, int height, int width) {
    return x >= 0 && x < height && y >= 0 && y < width;
}

long dijkstra(char grid[MAX_SIZE][MAX_SIZE], int height, int width, int cost1, int cost2, int costBomb) {
    long costs[MAX_SIZE][MAX_SIZE];
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            costs[i][j] = LONG_MAX; // Initialize costs to infinity
        }
    }

    PriorityQueue pq;
    initQueue(&pq, height * width);
    
    int startX, startY, goalX, goalY;
    // Find start and goal positions
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            if (grid[i][j] == 's') {
                startX = i;
                startY = j;
            } else if (grid[i][j] == 'g') {
                goalX = i;
                goalY = j;
            }
        }
    }

    costs[startX][startY] = 0;
    push(&pq, (Node){startX, startY, 0});

    int directions[4][2] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}}; // Up, Down, Left, Right

    while (pq.size > 0) {
        Node current = pop(&pq);
        if (current.x == goalX && current.y == goalY) {
            freeQueue(&pq);
            return current.cost; // Return the minimum cost to reach the goal
        }

        for (int i = 0; i < 4; i++) {
            int newX = current.x + directions[i][0];
            int newY = current.y + directions[i][1];

            if (isValid(newX, newY, height, width)) {
                long newCost = current.cost;
                if (grid[newX][newY] == '*') {
                    newCost += costBomb; // Cost for bomb
                } else {
                    newCost += (grid[newX][newY] == 's' || grid[newX][newY] == 'g') ? 0 : cost1; // Cost for normal terrain
                }

                if (newCost < costs[newX][newY]) {
                    costs[newX][newY] = newCost;
                    push(&pq, (Node){newX, newY, newCost});
                }
            }
        }
    }

    freeQueue(&pq);
    return LONG_MAX; // If goal is unreachable
}

int main() {
    int height, width, cost1, cost2, costBomb;
    char grid[MAX_SIZE][MAX_SIZE];

    // Input reading (assumed to be done correctly as per specification)
    scanf("%d %d", &height, &width);
    scanf("%d %d %d", &cost1, &cost2, &costBomb);
    for (int i = 0; i < height; i++) {
        scanf("%s", grid[i]);
    }

    long result = dijkstra(grid, height, width, cost1, cost2, costBomb);
    if (result == LONG_MAX) {
        printf("%s\n", INF); // Output "INF" if unreachable
    } else {
        printf("%ld\n", result); // Output the minimum cost
    }

    return 0;
}
