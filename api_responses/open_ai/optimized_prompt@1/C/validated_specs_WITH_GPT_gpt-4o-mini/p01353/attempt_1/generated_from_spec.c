#include <stdio.h>

long long calculate_score(int n, int h[], int a[], int d[], int s[]) {
    // Check preconditions: n must be non-negative and within bounds
    if (n < 0 || n > 1000) { // Assuming a maximum size of 1000 for the arrays
        return -1;
    }

    // Check that all input values are non-negative
    for (int i = 0; i < n; i++) {
        if (h[i] < 0 || a[i] < 0 || d[i] < 0 || s[i] < 0) {
            return -1;
        }
    }

    // Initialize score and check the first entity's attack against others' defense
    long long score = 0;
    int first_attack = a[0];
    int first_defense = d[0];

    // Check if the first entity's attack is valid against subsequent entities
    for (int i = 1; i < n; i++) {
        if (first_attack <= d[i]) {
            return -1; // Condition violated
        }
    }

    // Calculate the score based on the entities
    for (int i = 0; i < n; i++) {
        // Update score only if the attack of the current entity exceeds the defense of the first entity
        if (a[i] > first_defense) {
            score += s[i];
        }

        // Check if the health of the first entity is less than or equal to the score
        if (h[0] <= score) {
            return -1; // Condition violated
        }
    }

    return score; // Return the computed score if all conditions are satisfied
}

int main() {
    // Example usage
    int n = 3;
    int h[] = {100, 50, 30};
    int a[] = {20, 10, 15};
    int d[] = {5, 5, 5};
    int s[] = {10, 20, 30};

    long long result = calculate_score(n, h, a, d, s);
    printf("%lld\n", result); // Output the result
    return 0;
}
