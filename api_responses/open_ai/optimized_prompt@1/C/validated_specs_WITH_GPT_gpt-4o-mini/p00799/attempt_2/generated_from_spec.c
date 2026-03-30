#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *usage_times;      // Array of usage times
    int *recharge_times;   // Array of recharge times
    int usage_count;       // Count of usages
    int recharge_count;    // Count of recharges
    int active;            // Active state (1 for active, 0 for inactive)
} Battery;

int simulate_battery_management(int num_batteries, int time_limit, Battery *batteries) {
    int total_usages = 0;
    int current_time = 0;

    // Iterate through each battery
    for (int i = 0; i < num_batteries; i++) {
        int usage_index = 0;
        int recharge_index = 0;

        // Process each battery's usage and recharge cycles
        while (current_time < time_limit) {
            // Check if the battery is active and has usage time left
            if (batteries[i].active && usage_index < batteries[i].usage_count) {
                // Use the battery
                current_time += batteries[i].usage_times[usage_index];
                if (current_time <= time_limit) {
                    total_usages++; // Count the usage if within time limit
                }
                usage_index++;
                batteries[i].active = (usage_index < batteries[i].usage_count); // Check if still active
            }

            // Check if we can recharge the battery
            if (!batteries[i].active && recharge_index < batteries[i].recharge_count) {
                current_time += batteries[i].recharge_times[recharge_index];
                if (current_time <= time_limit) {
                    batteries[i].active = 1; // Recharge the battery
                }
                recharge_index++;
            }

            // If the battery is inactive and no more recharges, break out
            if (!batteries[i].active && recharge_index >= batteries[i].recharge_count) {
                break;
            }
        }
    }

    return total_usages; // Return the total number of usages
}

int main() {
    int num_batteries, time_limit;

    // Read number of batteries and time limit
    if (scanf("%d %d", &num_batteries, &time_limit) != 2 || num_batteries <= 0 || time_limit <= 0) {
        return 1; // Terminate gracefully on invalid input
    }

    Battery *batteries = malloc(num_batteries * sizeof(Battery));
    for (int i = 0; i < num_batteries; i++) {
        // Read usage and recharge times for each battery
        int usage_count, recharge_count;
        scanf("%d %d", &usage_count, &recharge_count);
        
        batteries[i].usage_times = malloc(usage_count * sizeof(int));
        batteries[i].recharge_times = malloc(recharge_count * sizeof(int));
        batteries[i].usage_count = usage_count;
        batteries[i].recharge_count = recharge_count;
        batteries[i].active = 1; // Start as active

        for (int j = 0; j < usage_count; j++) {
            scanf("%d", &batteries[i].usage_times[j]);
        }
        for (int j = 0; j < recharge_count; j++) {
            scanf("%d", &batteries[i].recharge_times[j]);
        }
    }

    int total_usages = simulate_battery_management(num_batteries, time_limit, batteries);
    printf("%d\n", total_usages); // Output the total number of usages

    // Free allocated memory
    for (int i = 0; i < num_batteries; i++) {
        free(batteries[i].usage_times);
        free(batteries[i].recharge_times);
    }
    free(batteries);

    return 0;
}
