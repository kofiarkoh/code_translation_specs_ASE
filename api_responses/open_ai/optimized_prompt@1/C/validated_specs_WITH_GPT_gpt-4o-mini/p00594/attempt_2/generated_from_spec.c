#include <stdio.h>

#define MAX_INTEGERS 1000000

int main() {
    int numbers[MAX_INTEGERS];
    int count = 0;
    int input;

    // Read the first integer which indicates the number of subsequent integers
    scanf("%d", &input);
    
    // Read integers until a zero is encountered
    while (input != 0 && count < MAX_INTEGERS) {
        numbers[count++] = input;
        scanf("%d", &input);
    }

    // If no integers were read, output "NO COLOR"
    if (count == 0) {
        printf("NO COLOR\n");
        return 0;
    }

    int half_count = count / 2;
    int max_length = 0;
    int current_length = 1;
    int most_frequent = numbers[0];
    int found = 0;

    // Process the array to find the longest contiguous segment
    for (int i = 1; i < count; i++) {
        if (numbers[i] == numbers[i - 1]) {
            current_length++;
        } else {
            // Check if the current segment is the longest found
            if (current_length > max_length) {
                max_length = current_length;
                most_frequent = numbers[i - 1];
            }
            current_length = 1; // Reset for the new number
        }
    }

    // Final check for the last segment
    if (current_length > max_length) {
        max_length = current_length;
        most_frequent = numbers[count - 1];
    }

    // Output the result based on the longest segment found
    if (max_length > half_count) {
        printf("%d\n", most_frequent);
    } else {
        printf("NO COLOR\n");
    }

    return 0;
}
