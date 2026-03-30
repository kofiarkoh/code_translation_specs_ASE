#include <stdio.h>
#include <stdlib.h>

#define MAX_RECTANGLES 10

typedef struct {
    int x1, y1, x2, y2; // Coordinates of the rectangle
    int index;          // Original index for output order
} Rectangle;

// Function to check if two rectangles overlap
int doOverlap(Rectangle r1, Rectangle r2) {
    // Check if one rectangle is to the left of the other
    if (r1.x1 >= r2.x2 || r2.x1 >= r1.x2)
        return 0; // No overlap
    // Check if one rectangle is above the other
    if (r1.y1 >= r2.y2 || r2.y1 >= r1.y2)
        return 0; // No overlap
    return 1; // Overlap exists
}

// Comparison function for sorting rectangles based on their coordinates
int compareRectangles(const void *a, const void *b) {
    Rectangle *r1 = (Rectangle *)a;
    Rectangle *r2 = (Rectangle *)b;
    // Sort primarily by x1, then by y1
    if (r1->x1 != r2->x1)
        return r1->x1 - r2->x1;
    return r1->y1 - r2->y1;
}

// Main processing function
void processRectangles(int n, Rectangle rectangles[]) {
    // Sort rectangles based on their coordinates
    qsort(rectangles, n, sizeof(Rectangle), compareRectangles);
    
    // Output order array
    int order[MAX_RECTANGLES];
    int processed[MAX_RECTANGLES] = {0}; // To track processed rectangles
    int orderIndex = 0;

    // Determine the order of processing based on overlaps
    for (int i = 0; i < n; i++) {
        if (!processed[i]) {
            order[orderIndex++] = rectangles[i].index; // Mark as processed
            processed[i] = 1; // Mark this rectangle as used
            // Check for overlaps with other rectangles
            for (int j = i + 1; j < n; j++) {
                if (!processed[j] && doOverlap(rectangles[i], rectangles[j])) {
                    order[orderIndex++] = rectangles[j].index; // Process overlapping rectangle
                    processed[j] = 1; // Mark as used
                }
            }
        }
    }

    // Output the order of processing
    for (int i = 0; i < orderIndex; i++) {
        printf("%d ", order[i]);
    }
    printf("\n");
}

int main() {
    int n; // Number of rectangles
    Rectangle rectangles[MAX_RECTANGLES];

    // Read input
    while (scanf("%d", &n) == 1 && n > 0 && n <= MAX_RECTANGLES) {
        for (int i = 0; i < n; i++) {
            scanf("%d %d %d %d", &rectangles[i].x1, &rectangles[i].y1, &rectangles[i].x2, &rectangles[i].y2);
            rectangles[i].index = i; // Store original index
        }
        processRectangles(n, rectangles); // Process the rectangles
    }

    return 0;
}
