#include <stdio.h>
#include <stdlib.h>

#define MAX_POINTS 1000000 // Define a maximum number of points

typedef struct {
    int x, y;
    int id; // Unique ID for each point
} Point;

typedef struct Node {
    Point point;
    struct Node *left, *right;
} Node;

Node* build_kd_tree(Point* points, int start, int end, int depth) {
    if (start > end) return NULL;

    // Sort points based on depth (0 for x-axis, 1 for y-axis)
    int axis = depth % 2;
    // Simple selection sort for demonstration; can be optimized
    for (int i = start; i < end; i++) {
        for (int j = i + 1; j <= end; j++) {
            if ((axis == 0 && points[i].x > points[j].x) || 
                (axis == 1 && points[i].y > points[j].y)) {
                Point temp = points[i];
                points[i] = points[j];
                points[j] = temp;
            }
        }
    }

    int mid = (start + end) / 2;
    Node* node = (Node*)malloc(sizeof(Node));
    node->point = points[mid];
    node->left = build_kd_tree(points, start, mid - 1, depth + 1);
    node->right = build_kd_tree(points, mid + 1, end, depth + 1);
    return node;
}

void range_query(Node* node, int sx, int tx, int sy, int ty, int* result, int* count) {
    if (node == NULL) return;

    // Check if the point is within the bounds
    if (node->point.x >= sx && node->point.x <= tx && 
        node->point.y >= sy && node->point.y <= ty) {
        result[(*count)++] = node->point.id; // Store the point ID
    }

    // Traverse the tree based on the current depth
    if (node->left != NULL && (node->point.x > sx)) {
        range_query(node->left, sx, tx, sy, ty, result, count);
    }
    if (node->right != NULL && (node->point.x < tx)) {
        range_query(node->right, sx, tx, sy, ty, result, count);
    }
}

int compare(const void* a, const void* b) {
    return (*(int*)a - *(int*)b; // Comparison function for sorting IDs
}

int main() {
    int n;
    scanf("%d", &n);
    if (n <= 0 || n > MAX_POINTS) return 0; // Handle invalid n

    Point* points = (Point*)malloc(n * sizeof(Point));
    for (int i = 0; i < n; i++) {
        scanf("%d %d", &points[i].x, &points[i].y);
        points[i].id = i; // Assign unique ID
    }

    Node* root = build_kd_tree(points, 0, n - 1, 0);
    free(points); // Free the original points array after building the tree

    int q;
    scanf("%d", &q);
    for (int i = 0; i < q; i++) {
        int sx, tx, sy, ty;
        scanf("%d %d %d %d", &sx, &tx, &sy, &ty);
        int* result = (int*)malloc(n * sizeof(int)); // Allocate max size for results
        int count = 0;

        range_query(root, sx, tx, sy, ty, result, &count);
        if (count > 0) {
            qsort(result, count, sizeof(int), compare); // Sort the results
            for (int j = 0; j < count; j++) {
                printf("%d ", result[j]);
            }
            printf("\n");
        } else {
            printf("\n"); // Empty output for no points found
        }
        free(result); // Free the result array
    }

    // Free the k-d tree (not implemented here for brevity)
    return 0;
}
