#include <stdio.h>
#include <stdlib.h>

int maxHistogramArea(int* heights, int n) {
    // Stack to store the indices of the histogram bars
    int* stack = (int*)malloc(n * sizeof(int));
    int top = -1; // Stack pointer
    int maxArea = 0; // Variable to store the maximum area
    int i = 0; // Current index

    while (i < n) {
        // If this bar is higher than the bar at stack top, push it to the stack
        if (top == -1 || heights[stack[top]] <= heights[i]) {
            stack[++top] = i++;
        } else {
            // Pop the top
            int topIndex = stack[top--];
            // Calculate the area with heights[topIndex] as the smallest (or minimum height) bar 'h'
            int area = heights[topIndex] * (top == -1 ? i : i - stack[top] - 1);
            // Update max area, if needed
            if (area > maxArea) {
                maxArea = area;
            }
        }
    }

    // Now pop the remaining bars from stack and calculate area with each popped bar
    while (top != -1) {
        int topIndex = stack[top--];
        int area = heights[topIndex] * (top == -1 ? i : i - stack[top] - 1);
        if (area > maxArea) {
            maxArea = area;
        }
    }

    free(stack); // Free the allocated memory for the stack
    return maxArea; // Return the maximum area found
}

int main() {
    int n;
    // Read the number of histogram bars
    if (scanf("%d", &n) != 1 || n < 0) {
        // Undefined behavior for negative input
        return 1; // Exit with error code
    }

    int* heights = (int*)malloc(n * sizeof(int));
    // Read the heights of the histogram bars
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &heights[i]) != 1 || heights[i] < 0) {
            // Undefined behavior for invalid heights
            free(heights);
            return 1; // Exit with error code
        }
    }

    // Compute the maximum rectangular area
    int maxArea = maxHistogramArea(heights, n);
    printf("%d\n", maxArea); // Output the maximum area

    free(heights); // Free the allocated memory for heights
    return 0; // Successful execution
}
