#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_LENGTH 10

// Function to extract numbers from the input string
int extract_numbers(const char *input, int *numbers) {
    int count = 0;
    char buffer[10]; // Buffer to hold the current number as a string
    int index = 0;

    for (int i = 0; input[i] != '\0' && count < MAX_LENGTH; i++) {
        if (isdigit(input[i])) {
            buffer[index++] = input[i]; // Collect digits
        } else {
            if (index > 0) {
                buffer[index] = '\0'; // Null-terminate the string
                numbers[count++] = atoi(buffer); // Convert to integer
                index = 0; // Reset buffer index
            }
        }
    }
    // Handle the last number if the string ends with digits
    if (index > 0) {
        buffer[index] = '\0';
        numbers[count++] = atoi(buffer);
    }
    return count; // Return the count of extracted numbers
}

// Function to find the maximum sum of selected numbers
void find_max_sum(int target, int *numbers, int count) {
    int max_sum = 0;
    int best_mask = 0;
    int found_valid = 0;
    int rejected = 0;

    // Iterate through all possible combinations using bitmask
    for (int mask = 1; mask < (1 << count); mask++) {
        int current_sum = 0;
        int current_mask = mask;

        for (int j = 0; j < count; j++) {
            if (mask & (1 << j)) {
                current_sum += numbers[j];
            }
        }

        // Check if the current sum is valid
        if (current_sum <= target) {
            found_valid = 1;
            if (current_sum > max_sum) {
                max_sum = current_sum;
                best_mask = mask;
                rejected = 0; // Reset rejected if we found a new max
            } else if (current_sum == max_sum) {
                rejected = 1; // Mark as rejected if we have a tie
            }
        }
    }

    // Output results based on findings
    if (!found_valid) {
        printf("error\n");
    } else {
        if (rejected) {
            printf("rejected\n");
        } else {
            printf("%d\n", max_sum);
            for (int j = 0; j < count; j++) {
                if (best_mask & (1 << j)) {
                    printf("%d ", numbers[j]);
                }
            }
            printf("\n");
        }
    }
}

int main() {
    int target;
    char input[MAX_LENGTH + 1];
    int numbers[MAX_LENGTH];

    // Read target sum
    scanf("%d", &target);
    // Read input string
    scanf("%s", input);

    // Extract numbers from the input string
    int count = extract_numbers(input, numbers);

    // Find and print the maximum sum and selected numbers
    find_max_sum(target, numbers, count);

    return 0;
}
