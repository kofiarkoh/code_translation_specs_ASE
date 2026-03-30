#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int weight;
    int value;
    double ratio; // value-to-weight ratio
} Item;

// Comparator function for sorting items based on value-to-weight ratio
int compare(const void *a, const void *b) {
    Item *itemA = (Item *)a;
    Item *itemB = (Item *)b;
    // Sort in descending order of ratio
    return (itemB->ratio > itemA->ratio) - (itemB->ratio < itemA->ratio);
}

double fractional_knapsack(int n, double max_weight, int weights[], int values[]) {
    if (n <= 0 || max_weight <= 0.0) {
        return 0.0; // If no items or capacity is zero, return 0
    }

    Item *items = malloc(n * sizeof(Item));
    if (!items) {
        return 0.0; // Handle memory allocation failure
    }

    // Initialize items and calculate value-to-weight ratios
    for (int i = 0; i < n; i++) {
        items[i].weight = weights[i];
        items[i].value = values[i];
        if (items[i].weight != 0) {
            items[i].ratio = (double)items[i].value / items[i].weight; // Calculate ratio
        } else {
            items[i].ratio = (items[i].value >= 0) ? 0.0 : -1.0; // Handle zero weight
        }
    }

    // Sort items based on value-to-weight ratio
    qsort(items, n, sizeof(Item), compare);

    double total_value = 0.0;
    double remaining_capacity = max_weight;

    // Process items
    for (int i = 0; i < n; i++) {
        if (items[i].weight < 0) {
            // Negative weight increases capacity
            remaining_capacity += -items[i].weight;
            if (items[i].value < 0) {
                total_value += items[i].value; // Negative value decreases total value
            }
            continue; // Skip to next item
        }

        if (items[i].value <= 0 || items[i].weight > remaining_capacity) {
            continue; // Ignore non-positive value items or items that can't fit
        }

        // If the item can be fully added
        if (items[i].weight <= remaining_capacity) {
            total_value += items[i].value;
            remaining_capacity -= items[i].weight;
        } else {
            // Add a fraction of the item
            total_value += items[i].value * (remaining_capacity / items[i].weight);
            break; // Knapsack is full
        }
    }

    free(items); // Free allocated memory
    return total_value < 0 ? 0.0 : total_value; // Ensure non-negative output
}

int main() {
    int n = 3; // Number of items
    double max_weight = 50.0; // Maximum weight capacity
    int weights[] = {10, 20, -5}; // Item weights
    int values[] = {60, 100, -10}; // Item values

    double max_value = fractional_knapsack(n, max_weight, weights, values);
    printf("Maximum value in the knapsack: %.2f\n", max_value);
    return 0;
}
