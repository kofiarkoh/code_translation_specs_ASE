#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int weight;
    int size;
    int value;
} Item;

int compute_max_value(int num_items, Item* items, int max_size) {
    // Dynamic programming array to store maximum values for each size
    int* dp = (int*)malloc((max_size + 1) * sizeof(int));
    if (dp == NULL) {
        // Memory allocation failed, terminate without valid output
        return 0; // Undefined behavior as per specification
    }

    // Initialize the dp array
    for (int i = 0; i <= max_size; i++) {
        dp[i] = 0;
    }

    // Process each item
    for (int i = 0; i < num_items; i++) {
        // Traverse dp array backwards to avoid overwriting values
        for (int j = max_size; j >= items[i].size; j--) {
            // Update the dp array with the maximum value obtainable
            dp[j] = (dp[j] > dp[j - items[i].size] + items[i].value) ? dp[j] : (dp[j - items[i].size] + items[i].value);
        }
    }

    // The maximum obtainable value is found at dp[max_size]
    int max_value = dp[max_size];

    // Free the allocated memory for dp array
    free(dp);

    return max_value;
}

int main() {
    int num_items;
    // Read the number of items
    if (scanf("%d", &num_items) != 1 || num_items < 0) {
        return 1; // Input reading failure, undefined behavior
    }

    Item* items = (Item*)malloc(num_items * sizeof(Item));
    if (items == NULL) {
        return 0; // Memory allocation failed, terminate without valid output
    }

    // Read each item's weight, size, and value
    for (int i = 0; i < num_items; i++) {
        if (scanf("%d %d %d", &items[i].weight, &items[i].size, &items[i].value) != 3) {
            free(items);
            return 1; // Input reading failure, undefined behavior
        }
    }

    int max_size;
    // Read the maximum size limit
    if (scanf("%d", &max_size) != 1 || max_size < 0) {
        free(items);
        return 1; // Input reading failure, undefined behavior
    }

    // Compute the maximum obtainable value
    int result = compute_max_value(num_items, items, max_size);
    printf("%d\n", result);

    // Free the allocated memory for items
    free(items);

    return 0;
}
