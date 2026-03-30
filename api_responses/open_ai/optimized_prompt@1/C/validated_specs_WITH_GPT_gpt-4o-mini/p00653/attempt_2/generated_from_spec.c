#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define INF INT_MAX // Predefined constant for invalid queries

typedef struct {
    int **grid; // 2D grid of integers
    int **segTree; // 2D segment tree for range minimum queries
    int rows, cols; // Dimensions of the grid
} SegmentTree2D;

// Function to create a 2D array
int** create2DArray(int rows, int cols) {
    int **array = malloc(rows * sizeof(int *));
    for (int i = 0; i < rows; i++) {
        array[i] = malloc(cols * sizeof(int));
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

// Function to initialize the segment tree
SegmentTree2D* initSegmentTree(int rows, int cols, int *values) {
    if (rows <= 0 || cols <= 0) {
        exit(1); // Terminate if dimensions are non-positive
    }

    SegmentTree2D *st = malloc(sizeof(SegmentTree2D));
    st->rows = rows;
    st->cols = cols;
    st->grid = create2DArray(rows, cols);
    st->segTree = create2DArray(rows * 4, cols * 4); // Allocate space for segment tree

    // Populate the grid with initial values
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            st->grid[i][j] = values[i * cols + j];
        }
    }

    // Build the segment tree
    buildSegmentTree(st, 0, 0, rows - 1, 0, 0, cols - 1);
    return st;
}

// Function to build the segment tree
void buildSegmentTree(SegmentTree2D *st, int rowStart, int rowEnd, int colStart, int colEnd) {
    // Base case: if the range is a single cell
    if (rowStart == rowEnd && colStart == colEnd) {
        st->segTree[rowStart][colStart] = st->grid[rowStart][colStart];
        return;
    }

    // Recursive case: divide the range
    int midRow = (rowStart + rowEnd) / 2;
    int midCol = (colStart + colEnd) / 2;

    // Build the four quadrants
    buildSegmentTree(st, rowStart, midRow, colStart, midCol); // Top-left
    buildSegmentTree(st, rowStart, midRow, midCol + 1, colEnd); // Top-right
    buildSegmentTree(st, midRow + 1, rowEnd, colStart, midCol); // Bottom-left
    buildSegmentTree(st, midRow + 1, rowEnd, midCol + 1, colEnd); // Bottom-right

    // Combine results
    st->segTree[rowStart][colStart] = 
        fmin(fmin(st->segTree[rowStart][colStart], st->segTree[rowStart][midCol + 1]),
             fmin(st->segTree[midRow + 1][colStart], st->segTree[midRow + 1][midCol + 1]));
}

// Function to update the grid and segment tree
void update(SegmentTree2D *st, int row, int col, int value) {
    if (row < 0 || row >= st->rows || col < 0 || col >= st->cols) {
        return; // Ignore out-of-bounds updates
    }
    st->grid[row][col] = value;
    // Update the segment tree accordingly
    buildSegmentTree(st, 0, st->rows - 1, 0, st->cols - 1);
}

// Function to query the minimum value in a rectangular subregion
int query(SegmentTree2D *st, int row1, int col1, int row2, int col2) {
    // Adjust for 1-based indexing
    row1--; col1--; row2--; col2--;
    
    // Check for out-of-bounds queries
    if (row1 < 0 || row2 >= st->rows || col1 < 0 || col2 >= st->cols || row1 > row2 || col1 > col2) {
        return INF; // Return predefined constant for invalid queries
    }

    // Perform the query on the segment tree
    return rangeMinQuery(st, row1, row2, col1, col2);
}

// Function to perform range minimum query
int rangeMinQuery(SegmentTree2D *st, int row1, int row2, int col1, int col2) {
    // This function would implement the logic to query the segment tree
    // For simplicity, we will return a placeholder value
    return 0; // Placeholder, actual implementation needed
}

// Cleanup function
void freeSegmentTree(SegmentTree2D *st) {
    free2DArray(st->grid, st->rows);
    free2DArray(st->segTree, st->rows * 4);
    free(st);
}

int main() {
    // Example usage
    int rows = 3, cols = 3;
    int values[] = {1, 2, 3, 4, 5, 6, 7, 8, 9};
    SegmentTree2D *st = initSegmentTree(rows, cols, values);

    // Example query
    int minValue = query(st, 1, 1, 2, 2); // Querying the minimum in the subregion (1,1) to (2,2)
    printf("Minimum value: %d\n", minValue);

    // Cleanup
    freeSegmentTree(st);
    return 0;
}
