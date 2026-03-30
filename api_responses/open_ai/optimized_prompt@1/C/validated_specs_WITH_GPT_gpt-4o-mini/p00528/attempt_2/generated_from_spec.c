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

void dijkstra(PointList *points, int startIndex, int goalIndex) {
    MinHeap heap;
    initMinHeap(&heap, points->size);
    heap.distances[startIndex] = 0;

    while (1) {
        int currentIndex = minHeapExtractMin(&heap);
        if (currentIndex == -1) break; // No more nodes to process

        // Check if we reached the goal
        if (currentIndex == goalIndex) {
            printf("%d\n", heap.distances[currentIndex]);
            freeMinHeap(&heap);
            return;
        }

        // Process neighbors (in this case, we assume direct neighbors)
        for (int i = 0; i < points->size; i++) {
            if (i != currentIndex && !heap.visited[i]) {
                // Assuming a distance of 1 for adjacent points
                int newDist = heap.distances[currentIndex] + 1;
                if (newDist < heap.distances[i]) {
                    heap.distances[i] = newDist;
                }
            }
        }
    }

    // If we exit the loop without finding the goal
    printf("-1\n");
    freeMinHeap(&heap);
}

int main() {
    int m, n, k;
    scanf("%d %d %d", &m, &n, &k);

    PointList points;
    initPointList(&points, k + 2); // +2 for potential start and goal points

    for (int i = 0; i < k; i++) {
        int x, y;
        scanf("%d %d", &x, &y);
        addPoint(&points, x, y);
    }

    // Define starting point (1, 1)
    int startIndex = -1;
    for (int i = 0; i < points.size; i++) {
        if (points.points[i].x == 1 && points.points[i].y == 1) {
            startIndex = i;
            break;
        }
    }
    if (startIndex == -1) {
        addPoint(&points, 1, 1);
        startIndex = points.size - 1; // New point added at the end
    }

    // Define goal point (m, n)
    int goalIndex = -1;
    for (int i = 0; i < points.size; i++) {
        if (points.points[i].x == m && points.points[i].y == n) {
            goalIndex = i;
            break;
        }
    }
    if (goalIndex == -1) {
        addPoint(&points, m, n);
        goalIndex = points.size - 1; // New point added at the end
    }

    // If both points are not defined, return -1
    if (startIndex == -1 || goalIndex == -1) {
        printf("-1\n");
    } else {
        dijkstra(&points, startIndex, goalIndex);
    }

    free(points.points);
    return 0;
}
