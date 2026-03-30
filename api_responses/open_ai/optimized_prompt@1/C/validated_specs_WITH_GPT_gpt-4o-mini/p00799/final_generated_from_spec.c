#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *usage_times;      // Array of usage times for the battery
    int *recharge_times;   // Array of recharge times for the battery
    int usage_count;       // Number of usages recorded
    int recharge_count;    // Number of recharges recorded
    int active;            // Active state of the battery (1 for active, 0 for inactive)
} Battery;

int simulate_battery_management(int num_batteries, int time_limit, Battery *batteries) {
    int total_usages = 0;
    int current_time = 0;

    // Loop until the time limit is reached
    while (current_time < time_limit) {
        for (int i = 0; i < num_batteries; i++) {
            if (batteries[i].active) {
                // If the battery is active, use it
                if (batteries[i].usage_count < sizeof(batteries[i].usage_times) / sizeof(int)) {
                    int usage_time = batteries[i].usage_times[batteries[i].usage_count];
                    if (current_time + usage_time <= time_limit) {
                        total_usages++; // Count the usage
                        current_time += usage_time; // Increment current time
                        batteries[i].usage_count++; // Move to the next usage time
                    } else {
                        // If usage time exceeds time limit, break out
                        break;
                    }
                }
                // Check if the battery needs to be recharged
                if (batteries[i].usage_count >= sizeof(batteries[i].usage_times) / sizeof(int)) {
                    batteries[i].active = 0; // Set battery to inactive
                }
            } else {
                // If the battery is inactive, check for recharge
                if (batteries[i].recharge_count < sizeof(batteries[i].recharge_times) / sizeof(int)) {
                    int recharge_time = batteries[i].recharge_times[batteries[i].recharge_count];
                    if (current_time + recharge_time <= time_limit) {
                        current_time += recharge_time; // Increment current time
                        batteries[i].active = 1; // Set battery to active
                        batteries[i].recharge_count++; // Move to the next recharge time
                    } else {
                        // If recharge time exceeds time limit, break out
                        break;
                    }
                }
            }
        }
    }

    return total_usages; // Return the total number of usages
}

int main() {
    // Example usage
    int num_batteries = 2;
    int time_limit = 10;

    // Allocate memory for batteries
    Battery *batteries = malloc(num_batteries * sizeof(Battery));
    
    // Initialize batteries with usage and recharge times
    // Example initialization (should be replaced with actual input)
    batteries[0].usage_times = (int[]){2, 3}; // Battery 0 usage times
    batteries[0].recharge_times = (int[]){1}; // Battery 0 recharge times
    batteries[0].usage_count = 0;
    batteries[0].recharge_count = 0;
    batteries[0].active = 1; // Initially active

    batteries[1].usage_times = (int[]){1, 2}; // Battery 1 usage times
    batteries[1].recharge_times = (int[]){2}; // Battery 1 recharge times
    batteries[1].usage_count = 0;
    batteries[1].recharge_count = 0;
    batteries[1].active = 1; // Initially active

    // Run the simulation
    int total_usages = simulate_battery_management(num_batteries, time_limit, batteries);
    
    // Output the total number of usages
    printf("Total battery usages: %d\n", total_usages);

    // Free allocated memory
    free(batteries);
    
    return 0;
}
