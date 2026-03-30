#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 20000

typedef struct {
    int weight;
    int size;
    int value;
} Item;

int max_value(int num_items, Item* items) {
    // Dynamic programming array to store maximum values for each size
    int* dp = (int*)malloc((MAX_SIZE + 1) * sizeof(int));
    if (dp == NULL) {
        // Memory allocation failed for dp array
        return 0; // Terminate without valid output
    }

    // Initialize the dp array
    for (int i = 0; i <= MAX_SIZE; i++) {
        dp[i] = 0;
    }

    // Process each item
    for (int i = 0; i < num_items; i++) {
        // Traverse dp array backwards to avoid overwriting values
        for (int j = MAX_SIZE; j >= items[i].size; j--) {
            // Update the dp array with the maximum value obtainable
            dp[j] = (dp[j] > dp[j - items[i].size] + items[i].value) ? dp[j] : (dp[j - items[i].size] + items[i].value);
        }
    }

    // Get the maximum value obtainable
    int result = dp[MAX_SIZE];

    // Free the allocated memory for dp array
    free(dp);

    return result;
}

int main() {
    int num_items;
    // Read the number of items
    if (scanf("%d", &num_items) != 1 || num_items < 0) {
        // Input reading failure or negative number of items
        return 1; // Exit with error
    }

    // Allocate memory for items
    Item* items = (Item*)malloc(num_items * sizeof(Item));
    if (items == NULL) {
        // Memory allocation failed for items
        return 1; // Exit with error
    }

    // Read each item's weight, size, and value
    for (int i = 0; i < num_items; i++) {
        if (scanf("%d %d %d", &items[i].weight, &items[i].size, &items[i].value) != 3 ||
            items[i].weight < 0 || items[i].size < 0 || items[i].value < 0) {
            // Input reading failure or negative item attributes
            free(items);
            return 1; // Exit with error
        }
    }

    // Compute the maximum obtainable value
    int result = max_value(num_items, items);

    // Output the result
    printf("%d\n", result);

    // Free the allocated memory for items
    free(items);

    return 0;
}
