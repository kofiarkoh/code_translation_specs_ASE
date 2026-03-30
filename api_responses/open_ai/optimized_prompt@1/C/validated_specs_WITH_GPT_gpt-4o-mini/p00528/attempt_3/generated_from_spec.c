#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

typedef struct {
    int x, y;
} Point;

typedef struct {
    Point point;
    int distance;
} Node;

typedef struct {
    Node *nodes;
    int size;
    int capacity;
} MinHeap;

MinHeap* createMinHeap(int capacity) {
    MinHeap *heap = (MinHeap *)malloc(sizeof(MinHeap));
    heap->nodes = (Node *)malloc(capacity * sizeof(Node));
    heap->size = 0;
    heap->capacity = capacity;
    return heap;
}

void swap(Node *a, Node *b) {
    Node temp = *a;
    *a = *b;
    *b = temp;
}

void minHeapify(MinHeap *heap, int idx) {
    int smallest = idx;
    int left = 2 * idx + 1;
    int right = 2 * idx + 2;

    if (left < heap->size && heap->nodes[left].distance < heap->nodes[smallest].distance) {
        smallest = left;
    }
    if (right < heap->size && heap->nodes[right].distance < heap->nodes[smallest].distance) {
        smallest = right;
    }
    if (smallest != idx) {
        swap(&heap->nodes[idx], &heap->nodes[smallest]);
        minHeapify(heap, smallest);
    }
}

void insertMinHeap(MinHeap *heap, Node node) {
    if (heap->size == heap->capacity) {
        return; // Heap is full
    }
    heap->nodes[heap->size] = node;
    int i = heap->size;
    heap->size++;

    while (i != 0 && heap->nodes[(i - 1) / 2].distance > heap->nodes[i].distance) {
        swap(&heap->nodes[i], &heap->nodes[(i - 1) / 2]);
        i = (i - 1) / 2;
    }
}

Node extractMin(MinHeap *heap) {
    if (heap->size == 0) {
        Node nullNode = {{-1, -1}, INT_MAX}; // Return a node with max distance
        return nullNode;
    }
    Node root = heap->nodes[0];
    heap->nodes[0] = heap->nodes[heap->size - 1];
    heap->size--;
    minHeapify(heap, 0);
    return root;
}

int isValid(int x, int y, int m, int n) {
    return (x >= 1 && x <= m && y >= 1 && y <= n);
}

int calculateDistance(Point a, Point b) {
    return abs(a.x - b.x) + abs(a.y - b.y);
}

int dijkstra(int m, int n, int k, Point *specialPoints) {
    MinHeap *minHeap = createMinHeap(k + 2); // +2 for potential start and goal points
    int distances[m + 1][n + 1];
    int visited[m + 1][n + 1];

    for (int i = 0; i <= m; i++) {
        for (int j = 0; j <= n; j++) {
            distances[i][j] = INT_MAX; // Initialize distances
            visited[i][j] = 0; // Initialize visited
        }
    }

    Point start = {1, 1};
    Point goal = {m, n};
    int startFound = 0, goalFound = 0;

    for (int i = 0; i < k; i++) {
        if (specialPoints[i].x == start.x && specialPoints[i].y == start.y) {
            startFound = 1;
        }
        if (specialPoints[i].x == goal.x && specialPoints[i].y == goal.y) {
            goalFound = 1;
        }
    }

    if (!startFound) {
        specialPoints[k++] = start; // Add start point if not found
    }
    if (!goalFound) {
        specialPoints[k++] = goal; // Add goal point if not found
    }

    if (k < 2) {
        return -1; // Both points are not defined
    }

    distances[start.x][start.y] = 0;
    insertMinHeap(minHeap, (Node){start, 0});

    while (minHeap->size > 0) {
        Node current = extractMin(minHeap);
        int x = current.point.x;
        int y = current.point.y;

        if (visited[x][y]) {
            continue; // Skip if already visited
        }
        visited[x][y] = 1;

        if (current.point.x == goal.x && current.point.y == goal.y) {
            return distances[x][y]; // Return distance to goal
        }

        // Check adjacent points
        for (int i = 0; i < k; i++) {
            Point neighbor = specialPoints[i];
            if (isValid(neighbor.x, neighbor.y, m, n) && !visited[neighbor.x][neighbor.y]) {
                int newDist = distances[x][y] + calculateDistance(current.point, neighbor);
                if (newDist < distances[neighbor.x][neighbor.y]) {
                    distances[neighbor.x][neighbor.y] = newDist;
                    insertMinHeap(minHeap, (Node){neighbor, newDist});
                }
            }
        }
    }

    return -1; // No path found
}

int main() {
    int m, n, k;
    scanf("%d %d %d", &m, &n, &k);
    Point *specialPoints = (Point *)malloc(k * sizeof(Point));

    for (int i = 0; i < k; i++) {
        scanf("%d %d", &specialPoints[i].x, &specialPoints[i].y);
    }

    int result = dijkstra(m, n, k, specialPoints);
    printf("%d\n", result);

    free(specialPoints);
    return 0;
}
