#include <stdio.h>
#include <stdlib.h>

#define MAX_RECTANGLES 10

typedef struct {
    int x1, y1, x2, y2;
    int index; // To keep track of the original order
} Rectangle;

// Function to compare rectangles based on the specified overlap rules
int compare_rectangles(const void *a, const void *b) {
    Rectangle *rectA = (Rectangle *)a;
    Rectangle *rectB = (Rectangle *)b;

    // Check for overlap
    if (rectA->x1 < rectB->x2 && rectA->x2 > rectB->x1 && rectA->y1 < rectB->y2 && rectA->y2 > rectB->y1) {
        // If they overlap, sort by y2, then by x1
        if (rectA->y2 != rectB->y2) {
            return rectA->y2 - rectB->y2; // Lower y2 comes first
        }
        return rectA->x1 - rectB->x1; // If y2 is the same, sort by x1
    }
    return 0; // No overlap, keep original order
}

// Function to process rectangles and determine their order
void process_rectangles(Rectangle rectangles[], int count) {
    int processed[MAX_RECTANGLES] = {0}; // To track processed rectangles
    int output[MAX_RECTANGLES]; // To store the order of processed rectangles
    int output_count = 0;

    // Sort rectangles based on the overlap rules
    qsort(rectangles, count, sizeof(Rectangle), compare_rectangles);

    for (int i = 0; i < count; i++) {
        if (!processed[i]) {
            // Mark the current rectangle as processed
            processed[i] = 1;
            output[output_count++] = rectangles[i].index; // Store the original index

            // Check for overlaps with subsequent rectangles
            for (int j = i + 1; j < count; j++) {
                if (!processed[j] && 
                    rectangles[i].x1 < rectangles[j].x2 && rectangles[i].x2 > rectangles[j].x1 &&
                    rectangles[i].y1 < rectangles[j].y2 && rectangles[i].y2 > rectangles[j].y1) {
                    processed[j] = 1; // Mark as processed due to overlap
                }
            }
        }
    }

    // Output the order of processed rectangles
    for (int i = 0; i < output_count; i++) {
        printf("%d ", output[i]);
    }
    printf("\n");
}

int main() {
    int n; // Number of rectangles
    Rectangle rectangles[MAX_RECTANGLES];

    // Read the number of rectangles
    scanf("%d", &n);
    for (int i = 0; i < n; i++) {
        // Read rectangle coordinates
        scanf("%d %d %d %d", &rectangles[i].x1, &rectangles[i].y1, &rectangles[i].x2, &rectangles[i].y2);
        rectangles[i].index = i; // Store the original index
    }

    // Process the rectangles to determine the order
    process_rectangles(rectangles, n);

    return 0;
}
