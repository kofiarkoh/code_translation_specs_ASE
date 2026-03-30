#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INF 2147483647 // Predefined constant for invalid queries

typedef struct {
    int **grid;        // 2D grid of integers
    int **segTree;    // 2D segment tree for range minimum queries
    int rows;         // Number of rows in the grid
    int cols;         // Number of columns in the grid
} SegmentTree2D;

// Function to create a 2D array
int** create2DArray(int rows, int cols) {
    int **array = (int **)malloc(rows * sizeof(int *));
    for (int i = 0; i < rows; i++) {
        array[i] = (int *)malloc(cols * sizeof(int));
    }
    return array;
}

// Function to free a 2D array
void free2DArray(int **array, int rows) {
    for (int i = 0; i < rows; i++) {
        free(array[i]);
    }
    free(array);
}

// Function to build the segment tree
void buildSegmentTree(SegmentTree2D *st) {
    for (int i = 0; i < st->rows; i++) {
        for (int j = 0; j < st->cols; j++) {
            st->segTree[i][j] = st->grid[i][j];
        }
    }
    // Additional logic to build the segment tree would go here
}

// Function to update the grid and segment tree
void update(SegmentTree2D *st, int x, int y, int value) {
    if (x < 0 || x >= st->rows || y < 0 || y >= st->cols) return; // Out of bounds
    st->grid[x][y] = value;
    // Logic to update the segment tree would go here
}

// Function to query the minimum value in a rectangular subregion
int query(SegmentTree2D *st, int x1, int y1, int x2, int y2) {
    // Check for out-of-bounds queries
    if (x1 < 0 || x2 >= st->rows || y1 < 0 || y2 >= st->cols || x1 > x2 || y1 > y2) {
        return INF; // Return predefined constant for invalid query
    }
    int minValue = INF; // Initialize minimum value
    for (int i = x1; i <= x2; i++) {
        for (int j = y1; j <= y2; j++) {
            if (st->grid[i][j] < minValue) {
                minValue = st->grid[i][j]; // Update minimum value
            }
        }
    }
    return minValue; // Return the minimum value found
}

// Main function to demonstrate the segment tree functionality
int main() {
    int rows, cols;
    scanf("%d %d", &rows, &cols);
    
    if (rows <= 0 || cols <= 0) {
        return 0; // Terminate if dimensions are non-positive
    }

    SegmentTree2D st;
    st.rows = rows;
    st.cols = cols;
    st.grid = create2DArray(rows, cols);
    st.segTree = create2DArray(rows, cols); // Allocate space for segment tree

    // Populate the grid
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            scanf("%d", &st.grid[i][j]);
        }
    }

    buildSegmentTree(&st); // Build the segment tree

    // Process queries
    int q;
    scanf("%d", &q); // Number of queries
    for (int i = 0; i < q; i++) {
        int x1, y1, x2, y2;
        scanf("%d %d %d %d", &x1, &y1, &x2, &y2);
        // Adjust for 1-based indexing
        int result = query(&st, x1 - 1, y1 - 1, x2 - 1, y2 - 1);
        printf("%d\n", result);
    }

    // Free allocated memory
    free2DArray(st.grid, rows);
    free2DArray(st.segTree, rows);
    
    return 0;
}
