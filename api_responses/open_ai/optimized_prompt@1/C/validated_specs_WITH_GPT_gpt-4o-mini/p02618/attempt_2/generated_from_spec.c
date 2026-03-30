#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define MAX_DAYS 100 // Assuming a maximum number of days for simplicity
#define MAX_TYPES 10 // Assuming a maximum number of types for simplicity

// Function to calculate the optimal schedule
void calculate_optimal_schedule(int D, int costs[], int satisfaction_scores[MAX_DAYS + 1][MAX_TYPES + 1], int optimal_schedule[]) {
    int current_score = 0;
    // Initialize the optimal schedule with random types
    for (int day = 1; day <= D; day++) {
        optimal_schedule[day] = rand() % MAX_TYPES + 1; // Random type selection (1 to type)
        current_score += satisfaction_scores[day][optimal_schedule[day]];
    }

    // Time constraint simulation (not actual timing logic)
    clock_t start_time = clock();
    while (((double)(clock() - start_time) / CLOCKS_PER_SEC) < 2.0) {
        int new_schedule[MAX_DAYS + 1];
        int new_score = 0;

        // Generate a new random schedule
        for (int day = 1; day <= D; day++) {
            new_schedule[day] = rand() % MAX_TYPES + 1; // Random type selection (1 to type)
            new_score += satisfaction_scores[day][new_schedule[day]];
        }

        // Update the schedule if the new score is higher
        if (new_score > current_score) {
            for (int day = 1; day <= D; day++) {
                optimal_schedule[day] = new_schedule[day];
            }
            current_score = new_score; // Update the current score
        }
    }
}

int main() {
    int D; // Number of days
    int costs[MAX_TYPES + 1]; // Costs array
    int satisfaction_scores[MAX_DAYS + 1][MAX_TYPES + 1]; // Satisfaction scores 2D array
    int optimal_schedule[MAX_DAYS + 1]; // Output schedule

    // Example initialization (in a real scenario, these would be provided as input)
    D = 5; // Example: 5 days
    for (int i = 1; i <= MAX_TYPES; i++) {
        costs[i] = rand() % 100; // Random costs for each type
    }
    for (int day = 1; day <= D; day++) {
        for (int type = 1; type <= MAX_TYPES; type++) {
            satisfaction_scores[day][type] = rand() % 100; // Random satisfaction scores
        }
    }

    srand(time(NULL)); // Seed for random number generation
    calculate_optimal_schedule(D, costs, satisfaction_scores, optimal_schedule);

    // Output the optimal schedule
    for (int day = 1; day <= D; day++) {
        printf("Day %d: Type %d\n", day, optimal_schedule[day]);
    }

    return 0;
}
