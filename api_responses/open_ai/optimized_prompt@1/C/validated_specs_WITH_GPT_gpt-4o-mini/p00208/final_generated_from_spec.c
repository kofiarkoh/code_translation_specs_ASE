#include <stdio.h>

int main() {
    int input;

    // Loop to read integers until a zero is encountered
    while (1) {
        scanf("%d", &input); // Read an integer from standard input

        // Check for the sentinel value to terminate
        if (input == 0) {
            break; // Exit the loop if the input is zero
        }

        // Array to store octal digits (maximum 11 digits for 32-bit integers)
        int octal[11] = {0}; // Initialize to zero as per the invariant
        int index = 0;

        // Convert the integer to octal representation
        while (input > 0) {
            octal[index++] = input % 8; // Store the octal digit
            input /= 8; // Reduce the number
        }

        // Apply transformation rules to octal digits
        for (int i = 0; i < index; i++) {
            if (octal[i] > 3) {
                octal[i] += 1; // Increment if greater than 3
            }
            if (octal[i] > 5) {
                octal[i] += 1; // Increment again if greater than 5
            }
        }

        // Output the modified octal representation
        for (int i = index - 1; i >= 0; i--) {
            printf("%d", octal[i]); // Print digits in reverse order
        }
        printf("\n"); // Newline after each output
    }

    return 0; // Successful termination of the program
}
