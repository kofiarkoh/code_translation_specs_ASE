#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_ITEMS 100

typedef struct {
    int value;      // Value of the item
    int energyCost; // Energy cost of the item
    int requiredEnergy; // Required energy to use the item
} Item;

Item items[MAX_ITEMS];
int memo[MAX_ITEMS][1001]; // Memoization array for storing results
int N; // Number of items
int D; // Target value

// Function to find the minimum number of items needed to reach the target value
int minItems(int target, int energy) {
    // Base case: if target is less than or equal to zero, no items are needed
    if (target <= 0) {
        return 0;
    }
    // If we have already computed this state, return the stored result
    if (memo[energy][target] != -1) {
        return memo[energy][target];
    }

    int minCount = MAX_ITEMS + 1; // Initialize to a large number

    // Try each item
    for (int i = 0; i < N; i++) {
        // Check if we can use this item (enough energy)
        if (energy >= items[i].requiredEnergy) {
            int result = minItems(target - items[i].value, energy - items[i].energyCost);
            if (result != -1) {
                minCount = (result + 1 < minCount) ? result + 1 : minCount; // Update minCount
            }
        }
    }

    // If minCount was updated, store it; otherwise, store -1 to indicate not achievable
    memo[energy][target] = (minCount == MAX_ITEMS + 1) ? -1 : minCount;
    return memo[energy][target];
}

int main() {
    // Read target value and number of items
    if (scanf("%d %d", &D, &N) != 2) {
        return 1; // Terminate with non-zero exit code on incorrect input format
    }

    // Handle special cases
    if (D <= 0) {
        printf("0\n"); // No items needed if target is less than or equal to zero
        return 0;
    }
    if (N < 0 || N > MAX_ITEMS) {
        return 1; // Terminate with non-zero exit code if N is out of bounds
    }

    // Read item values
    for (int i = 0; i < N; i++) {
        if (scanf("%d %d %d", &items[i].value, &items[i].energyCost, &items[i].requiredEnergy) != 3) {
            return 1; // Terminate with non-zero exit code on incorrect input format
        }
        // Ensure item attributes are non-negative
        if (items[i].value < 0 || items[i].energyCost < 0 || items[i].requiredEnergy < 0) {
            return 1; // Terminate with non-zero exit code on invalid item attributes
        }
    }

    // Initialize memoization array to -1
    memset(memo, -1, sizeof(memo));

    // Compute the minimum number of items needed
    int result = minItems(D, 1000); // Assuming a maximum energy level of 1000

    // Output the result
    if (result == -1) {
        printf("NA\n"); // Target value cannot be reached
    } else {
        printf("%d\n", result); // Output the minimum number of items needed
    }

    return 0;
}
