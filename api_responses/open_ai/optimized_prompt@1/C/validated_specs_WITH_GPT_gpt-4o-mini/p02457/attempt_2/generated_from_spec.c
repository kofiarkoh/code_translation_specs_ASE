#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *data;      // Pointer to the dynamic array
    int count;      // Current number of elements
    int capacity;   // Maximum capacity of the array
} DynamicArray;

// Function prototypes
DynamicArray* create_array(int capacity);
void free_array(DynamicArray *array);
int insert(DynamicArray *array, int x);
int find(DynamicArray *array, int x);
int delete(DynamicArray *array, int x);
void dump(DynamicArray *array, int x, int y);

int main() {
    int q;
    scanf("%d", &q); // Read number of operations
    DynamicArray *array = create_array(q);
    
    for (int i = 0; i < q; i++) {
        int com, x, y;
        scanf("%d", &com);
        if (com == 0) { // Insert
            scanf("%d", &x);
            printf("%d\n", insert(array, x));
        } else if (com == 1) { // Find
            scanf("%d", &x);
            printf("%d\n", find(array, x));
        } else if (com == 2) { // Delete
            scanf("%d", &x);
            delete(array, x);
        } else if (com == 3) { // Dump
            scanf("%d %d", &x, &y);
            dump(array, x, y);
        }
    }
    
    free_array(array);
    return 0;
}

DynamicArray* create_array(int capacity) {
    DynamicArray *array = (DynamicArray *)malloc(sizeof(DynamicArray));
    array->data = (int *)malloc(capacity * sizeof(int));
    array->count = 0;
    array->capacity = capacity;
    return array;
}

void free_array(DynamicArray *array) {
    free(array->data);
    free(array);
}

int insert(DynamicArray *array, int x) {
    if (array->count >= array->capacity) {
        return array->count; // Ignore if full
    }
    
    // Find the position to insert
    int pos = 0;
    while (pos < array->count && array->data[pos] < x) {
        pos++;
    }
    
    // Check for duplicates
    if (pos < array->count && array->data[pos] == x) {
        return array->count; // Ignore if duplicate
    }
    
    // Shift elements to make space
    for (int i = array->count; i > pos; i--) {
        array->data[i] = array->data[i - 1];
    }
    
    // Insert the new element
    array->data[pos] = x;
    array->count++;
    return array->count; // Return updated count
}

int find(DynamicArray *array, int x) {
    // Binary search for the element
    int left = 0, right = array->count - 1;
    while (left <= right) {
        int mid = left + (right - left) / 2;
        if (array->data[mid] == x) {
            return 1; // Found
        } else if (array->data[mid] < x) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return 0; // Not found
}

int delete(DynamicArray *array, int x) {
    // Find the position of the element to delete
    int pos = 0;
    while (pos < array->count && array->data[pos] < x) {
        pos++;
    }
    
    // If the element is not found, do nothing
    if (pos == array->count || array->data[pos] != x) {
        return; // Element not found
    }
    
    // Shift elements to remove the element
    for (int i = pos; i < array->count - 1; i++) {
        array->data[i] = array->data[i + 1];
    }
    
    array->count--; // Decrement count
}

void dump(DynamicArray *array, int x, int y) {
    for (int i = 0; i < array->count; i++) {
        if (array->data[i] >= x && array->data[i] <= y) {
            printf("%d ", array->data[i]);
        }
    }
    printf("\n"); // New line after dump
}
