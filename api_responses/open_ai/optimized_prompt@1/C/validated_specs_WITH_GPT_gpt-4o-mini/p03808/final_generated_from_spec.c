#include <stdio.h>

#define MAX_ELEMENTS 100001

int main() {
    int N; // Number of elements
    int numbers[MAX_ELEMENTS]; // Array to hold the integers
    long long sum = 0; // To maintain the sum of integers
    int differences[MAX_ELEMENTS - 1]; // Array to hold differences
    int condition_met = 1; // Flag to check if conditions are met

    // Read the number of elements
    scanf("%d", &N);

    // Preconditions check: N must be positive
    if (N <= 0 || N > MAX_ELEMENTS) {
        printf("NO\n");
        return 0;
    }

    // Read the sequence of integers
    for (int i = 0; i < N; i++) {
        scanf("%d", &numbers[i]);
        sum += numbers[i]; // Maintain the sum
    }

    // Check the divisibility condition
    if (sum % N != 0) {
        printf("NO\n");
        return 0;
    }

    // Calculate differences and check conditions
    for (int i = 1; i < N; i++) {
        differences[i - 1] = numbers[i] - numbers[i - 1]; // Calculate difference
        // Check if the difference is negative or exceeds a certain threshold
        if (differences[i - 1] < 0 || differences[i - 1] > 100) { // Assuming threshold is 100
            condition_met = 0; // Condition not met
            break;
        }
    }

    // Output based on the condition check
    if (condition_met) {
        printf("YES\n");
    } else {
        printf("NO\n");
    }

    return 0;
}
