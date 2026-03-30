#include <stdio.h>

void convert_to_octal_and_transform(int num) {
    // Array to hold octal digits, initialized to zero
    int octal_digits[11] = {0}; // Enough to hold octal representation of 32-bit int
    int index = 0;

    // Convert decimal to octal
    while (num > 0) {
        octal_digits[index++] = num % 8; // Store the octal digit
        num /= 8; // Reduce the number
    }

    // Output the transformed octal representation
    for (int i = index - 1; i >= 0; i--) {
        // Apply transformation rules to octal digits
        // Assuming transformation is to increment each digit by 1, wrapping around at 7
        int transformed_digit = (octal_digits[i] + 1) % 8;
        putchar(transformed_digit + '0'); // Output the transformed digit as a character
    }
    putchar('\n'); // Newline after each output
}

int main() {
    int input;

    // Read integers until a zero is encountered
    while (1) {
        scanf("%d", &input); // Read input integer
        if (input == 0) {
            break; // Terminate on sentinel value
        }
        convert_to_octal_and_transform(input); // Process non-zero input
    }

    return 0; // Successful termination
}
