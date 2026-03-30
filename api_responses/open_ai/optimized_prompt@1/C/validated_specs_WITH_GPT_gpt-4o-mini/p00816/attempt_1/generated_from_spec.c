#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_LENGTH 10

// Function to extract numbers from the input string
int extract_numbers(const char *input, int *numbers) {
    int count = 0;
    const char *ptr = input;
    while (*ptr && count < MAX_LENGTH) {
        if (isdigit(*ptr)) {
            numbers[count++] = atoi(ptr);
            while (isdigit(*ptr)) ptr++; // Skip the number
        } else {
            ptr++; // Skip non-numeric character
        }
    }
    return count; // Return the count of extracted numbers
}

// Function to compute the maximum sum based on the bitmask
void compute_max_sum(int target, int *numbers, int count) {
    int max_sum = 0;
    int best_mask = 0;
    int found_valid = 0;
    int multiple_max = 0;

    for (int mask = 1; mask < (1 << count); mask++) {
        int current_sum = 0;
        int current_mask = mask;
        for (int i = 0; i < count; i++) {
            if (mask & (1 << i)) {
                current_sum += numbers[i];
            }
        }
        if (current_sum <= target) {
            if (current_sum > max_sum) {
                max_sum = current_sum;
                best_mask = mask;
                found_valid = 1;
                multiple_max = 0; // Reset multiple max flag
            } else if (current_sum == max_sum) {
                multiple_max = 1; // Found another combination with the same max sum
            }
        }
    }

    // Output results based on findings
    if (!found_valid) {
        printf("error\n");
    } else if (multiple_max) {
        printf("rejected\n");
    } else {
        printf("%d\n", max_sum);
        for (int i = 0; i < count; i++) {
            if (best_mask & (1 << i)) {
                printf("%d ", numbers[i]);
            }
        }
        printf("\n");
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
    
    // Compute the maximum sum based on the extracted numbers
    compute_max_sum(target, numbers, count);

    return 0;
}
