#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *data;      // Array to hold the positions of the houses
    long *segTree;  // Segment tree for maximum value queries
    int size;       // Size of the segment tree
} HouseManager;

// Function to create a HouseManager instance
HouseManager* createHouseManager(int n) {
    HouseManager *manager = (HouseManager *)malloc(sizeof(HouseManager));
    manager->data = (int *)malloc(n * sizeof(int));
    manager->size = 1;
    while (manager->size < n) manager->size *= 2; // Ensure size is a power of two
    manager->segTree = (long *)malloc(2 * manager->size * sizeof(long));
    for (int i = 0; i < 2 * manager->size; i++) manager->segTree[i] = 0; // Initialize segment tree
    return manager;
}

// Function to update the segment tree with a new position
void update(HouseManager *manager, int index, int value) {
    manager->data[index] = value; // Update the position
    int pos = index + manager->size; // Position in segment tree
    manager->segTree[pos] = value; // Set value in segment tree
    // Recalculate the segment tree upwards
    while (pos > 1) {
        pos /= 2;
        manager->segTree[pos] = (manager->segTree[2 * pos] > manager->segTree[2 * pos + 1]) ? 
                                 manager->segTree[2 * pos] : manager->segTree[2 * pos + 1];
    }
}

// Function to query the maximum value in a range [l, r)
long queryMax(HouseManager *manager, int l, int r) {
    l += manager->size; // Adjust index for segment tree
    r += manager->size; // Adjust index for segment tree
    long maxVal = 0; // Initialize max value
    while (l < r) {
        if (l & 1) maxVal = (maxVal > manager->segTree[l]) ? maxVal : manager->segTree[l++]; // If l is odd
        if (r & 1) maxVal = (maxVal > manager->segTree[--r]) ? maxVal : manager->segTree[r]; // If r is odd
        l /= 2; // Move to parent
        r /= 2; // Move to parent
    }
    return maxVal;
}

// Function to calculate the total cost of moving houses
long calculateTotalCost(HouseManager *manager, int n) {
    long totalCost = 0; // Initialize total cost
    for (int i = 0; i < n; i++) {
        totalCost += manager->data[i]; // Sum positions for total cost
    }
    long maxInRange = queryMax(manager, 0, n); // Query max in the full range
    return totalCost - maxInRange; // Return the total cost minus the maximum value
}

// Main function to demonstrate the functionality
int main() {
    int n;
    scanf("%d", &n); // Read number of houses
    HouseManager *manager = createHouseManager(n);
    
    for (int i = 0; i < n; i++) {
        scanf("%d", &manager->data[i]); // Read positions
        update(manager, i, manager->data[i]); // Update segment tree
    }
    
    long totalCost = calculateTotalCost(manager, n); // Calculate total cost
    printf("%ld\n", totalCost); // Output the total cost
    
    // Free allocated memory
    free(manager->data);
    free(manager->segTree);
    free(manager);
    
    return 0;
}
