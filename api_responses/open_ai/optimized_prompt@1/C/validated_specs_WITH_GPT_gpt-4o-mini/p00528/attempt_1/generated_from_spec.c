#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

typedef struct {
    int x, y;
} Point;

typedef struct {
    Point *points;
    int size;
    int capacity;
} PointList;

typedef struct {
    int *distances;
    int *visited;
    int size;
} MinHeap;

void initPointList(PointList *list, int capacity) {
    list->points = (Point *)malloc(capacity * sizeof(Point));
    list->size = 0;
    list->capacity = capacity;
}

void addPoint(PointList *list, int x, int y) {
    if (list->size < list->capacity) {
        list->points[list->size++] = (Point){x, y};
    }
}

void initMinHeap(MinHeap *heap, int size) {
    heap->distances = (int *)malloc(size * sizeof(int));
    heap->visited = (int *)calloc(size, sizeof(int));
    heap->size = size;
    for (int i = 0; i < size; i++) {
        heap->distances[i] = INT_MAX; // Initialize distances to infinity
    }
}

void freeMinHeap(MinHeap *heap) {
    free(heap->distances);
    free(heap->visited);
}

int minHeapExtractMin(MinHeap *heap) {
    int minIndex = -1;
    int minValue = INT_MAX;
    for (int i = 0; i < heap->size; i++) {
        if (!heap->visited[i] && heap->distances[i] < minValue) {
            minValue = heap->distances[i];
            minIndex = i;
        }
    }
    if (minIndex != -1) {
        heap->visited[minIndex] = 1; // Mark as visited
    }
    return minIndex;
}

int dijkstra(PointList *points, int start, int goal) {
    MinHeap heap;
    initMinHeap(&heap, points->size);
    heap.distances[start] = 0;

    while (1) {
        int current = minHeapExtractMin(&heap);
        if (current == -1 || current == goal) {
            break; // No more nodes or reached goal
        }

        // Check neighbors (4 possible directions)
        for (int i = 0; i < points->size; i++) {
            if (i != current) {
                int dx = abs(points->points[current].x - points->points[i].x);
                int dy = abs(points->points[current].y - points->points[i].y);
                if (dx + dy == 1) { // Adjacent nodes
                    int newDist = heap.distances[current] + 1;
                    if (newDist < heap.distances[i]) {
                        heap.distances[i] = newDist;
                    }
                }
            }
        }
    }

    int result = heap.distances[goal];
    freeMinHeap(&heap);
    return (result == INT_MAX) ? -1 : result; // Return -1 if no path found
}

int main() {
    int m, n, k;
    scanf("%d %d %d", &m, &n, &k);
    
    if (m < 0 || n < 0 || k < 0) {
        printf("-1\n"); // Invalid input
        return -1;
    }

    PointList points;
    initPointList(&points, k);
    
    for (int i = 0; i < k; i++) {
        int x, y;
        scanf("%d %d", &x, &y);
        if (x >= 0 && x < m && y >= 0 && y < n) {
            addPoint(&points, x, y);
        }
    }

    if (points.size < 2) {
        printf("-1\n"); // Not enough points to define start and goal
        free(points.points);
        return -1;
    }

    int start = 0; // Assuming the first point is the start
    int goal = 1;  // Assuming the second point is the goal
    int distance = dijkstra(&points, start, goal);
    
    printf("%d\n", distance);
    free(points.points);
    return 0;
}
