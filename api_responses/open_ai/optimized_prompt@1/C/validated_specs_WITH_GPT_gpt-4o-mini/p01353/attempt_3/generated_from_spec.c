#include <stdio.h>

#define MAX_ENTITIES 40000

int main() {
    int n;
    int h[MAX_ENTITIES + 1], a[MAX_ENTITIES + 1], d[MAX_ENTITIES + 1], s[MAX_ENTITIES + 1];
    
    // Read the number of entities
    if (scanf("%d", &n) != 1 || n < 0 || n > MAX_ENTITIES) {
        printf("-1\n");
        return 0; // Invalid input for n
    }

    // Read the attributes for each entity
    for (int i = 0; i <= n; i++) {
        if (scanf("%d %d %d %d", &h[i], &a[i], &d[i], &s[i]) != 4 || 
            h[i] < 0 || a[i] < 0 || d[i] < 0 || s[i] < 0) {
            printf("-1\n");
            return 0; // Invalid input for entity attributes
        }
    }

    // Check the precondition for the first entity's attack value
    for (int i = 1; i <= n; i++) {
        if (a[0] <= d[i]) {
            printf("-1\n");
            return 0; // Precondition violated
        }
    }

    long long score = 0; // Initialize score
    long long total_turns = 0; // Initialize total turns

    // Calculate the score based on the entities
    for (int i = 1; i <= n; i++) {
        if (a[i] > d[0]) { // Check if attack exceeds defense of the first entity
            total_turns += (a[i] - d[0]); // Update total turns
            score += s[i]; // Update score
        }
    }

    // Check the health condition of the first entity
    if (h[0] <= score) {
        printf("-1\n");
        return 0; // Health condition violated
    }

    // Output the final score
    printf("%lld\n", score);
    return 0;
}
