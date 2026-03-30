#include <stdio.h>
#include <stdlib.h>

#define MAX_SETS 100 // Assuming a maximum number of input sets
#define MAX_TIMESTAMPS 3600 // Maximum possible unique timestamps in a 60-minute cycle

typedef struct {
    int hour;
    int minute;
    int second;
} TimeStamp;

// Function to check if the clock hands align at a given time
int handsAlign(int hour, int minute, int second, int offset) {
    // Calculate the positions of the hands
    int hourPos = (hour % 12) * 5 + (minute / 12); // Hour hand position in 60 divisions
    int minutePos = minute; // Minute hand position
    int secondPos = second / 12; // Second hand position in 60 divisions

    // Adjust positions by the offset
    hourPos = (hourPos + offset) % 60;
    minutePos = (minutePos + offset) % 60;
    secondPos = (secondPos + offset) % 60;

    // Check if all hands align
    return (hourPos == minutePos && minutePos == secondPos);
}

// Function to format and print the time
void printTime(int totalSeconds) {
    int hours = totalSeconds / 3600;
    int minutes = (totalSeconds % 3600) / 60;
    int seconds = totalSeconds % 60;
    printf("%02d:%02d:%02d\n", hours, minutes, seconds);
}

int main() {
    int n;
    scanf("%d", &n); // Read the number of sets

    for (int i = 0; i < n; i++) {
        int hour, minute, second;
        scanf("%d %d %d", &hour, &minute, &second); // Read clock hand positions

        int minTime = -1, maxTime = -1; // Initialize min and max times
        int foundAlignment = 0; // Flag to check if any alignment is found

        // Check for alignments for all minute offsets
        for (int offset = 0; offset < 60; offset++) {
            if (handsAlign(hour, minute, second, offset)) {
                int totalSeconds = (hour * 3600 + minute * 60 + second + offset) % 43200; // Total seconds in 12 hours
                if (!foundAlignment) {
                    minTime = totalSeconds; // Set min time on first alignment
                    maxTime = totalSeconds; // Set max time on first alignment
                    foundAlignment = 1; // Mark that we found at least one alignment
                } else {
                    if (totalSeconds < minTime) {
                        minTime = totalSeconds; // Update min time
                    }
                    if (totalSeconds > maxTime) {
                        maxTime = totalSeconds; // Update max time
                    }
                }
            }
        }

        // Output results
        if (foundAlignment) {
            printTime(minTime); // Print earliest alignment time
            printTime(maxTime); // Print latest alignment time
        } else {
            printf("No valid alignments\n"); // Indicate no alignments found
        }
    }

    return 0;
}
