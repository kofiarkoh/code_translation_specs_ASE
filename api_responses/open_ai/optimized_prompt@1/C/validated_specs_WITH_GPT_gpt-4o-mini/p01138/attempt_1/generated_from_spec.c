#include <stdio.h>

#define SECONDS_IN_A_DAY 86400 // 24 hours * 60 minutes * 60 seconds

int main() {
    int n; // Number of intervals
    while (scanf("%d", &n) == 1 && n > 0) {
        int time[SECONDS_IN_A_DAY] = {0}; // Array to track overlapping intervals

        for (int i = 0; i < n; i++) {
            int dep_h, dep_m, dep_s; // Departure time
            int arr_h, arr_m, arr_s; // Arrival time

            // Read departure and arrival times
            scanf("%d:%d:%d %d:%d:%d", &dep_h, &dep_m, &dep_s, &arr_h, &arr_m, &arr_s);

            // Convert times to seconds since start of the day
            int dep_time = dep_h * 3600 + dep_m * 60 + dep_s;
            int arr_time = arr_h * 3600 + arr_m * 60 + arr_s;

            // Increment the count for the departure time
            time[dep_time]++;
            // Decrement the count for the arrival time (exclusive)
            if (arr_time < SECONDS_IN_A_DAY) {
                time[arr_time]--;
            }
        }

        // Calculate the maximum number of overlapping intervals
        int max_overlap = 0;
        int current_overlap = 0;

        for (int j = 0; j < SECONDS_IN_A_DAY; j++) {
            current_overlap += time[j]; // Update current overlap
            if (current_overlap > max_overlap) {
                max_overlap = current_overlap; // Update max overlap if current exceeds
            }
        }

        // Output the maximum number of overlapping intervals
        printf("%d\n", max_overlap);
    }
    return 0; // Graceful termination
}
