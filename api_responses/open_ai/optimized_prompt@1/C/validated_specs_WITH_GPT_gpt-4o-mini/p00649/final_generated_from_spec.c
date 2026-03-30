#include <stdio.h>
#include <stdlib.h>

#define MAX_RECTANGLES 10

typedef struct {
    int x1, y1, x2, y2;
    int index; // To keep track of the original order
    int used;  // To mark if the rectangle has been processed
} Rectangle;

// Function to check if two rectangles overlap
int overlap(Rectangle a, Rectangle b) {
    return !(a.x1 >= b.x2 || a.x2 <= b.x1 || a.y1 >= b.y2 || a.y2 <= b.y1);
}

// Comparison function for sorting rectangles
int compare(const void *a, const void *b) {
    Rectangle *rectA = (Rectangle *)a;
    Rectangle *rectB = (Rectangle *)b;

    // First, check for overlap
    if (rectA->used && rectB->used) {
        // If both are used, we don't need to compare
        return 0;
    }
    
    // If one is used, the other should be processed first
    if (rectA->used) return 1;
    if (rectB->used) return -1;

    // Sort by y2, then by x1
    if (rectA->y2 != rectB->y2) {
        return rectA->y2 - rectB->y2;
    }
    return rectA->x1 - rectB->x1;
}

// Function to process rectangles and determine the order
void processRectangles(Rectangle rectangles[], int count) {
    int processedCount = 0;

    while (processedCount < count) {
        // Sort rectangles based on the defined rules
        qsort(rectangles, count, sizeof(Rectangle), compare);

        // Process the first rectangle that is not used
        for (int i = 0; i < count; i++) {
            if (!rectangles[i].used) {
                // Mark this rectangle as used
                rectangles[i].used = 1;
                printf("%d ", rectangles[i].index); // Output the index of the processed rectangle
                processedCount++;
                
                // Check for overlaps with other rectangles
                for (int j = 0; j < count; j++) {
                    if (!rectangles[j].used && overlap(rectangles[i], rectangles[j])) {
                        rectangles[j].used = 1; // Mark overlapping rectangles as used
                    }
                }
                break; // Break to re-sort after processing
            }
        }
    }
    printf("\n");
}

int main() {
    int n;
    Rectangle rectangles[MAX_RECTANGLES];

    // Read the number of rectangles
    while (scanf("%d", &n) == 1 && n > 0 && n <= MAX_RECTANGLES) {
        // Read each rectangle's coordinates
        for (int i = 0; i < n; i++) {
            scanf("%d %d %d %d", &rectangles[i].x1, &rectangles[i].y1, &rectangles[i].x2, &rectangles[i].y2);
            rectangles[i].index = i + 1; // Store the original index (1-based)
            rectangles[i].used = 0; // Initialize as not used
        }

        // Process the rectangles and output the order
        processRectangles(rectangles, n);
        
        // Reset used status for the next set of rectangles
        for (int i = 0; i < n; i++) {
            rectangles[i].used = 0;
        }
    }

    return 0;
}
