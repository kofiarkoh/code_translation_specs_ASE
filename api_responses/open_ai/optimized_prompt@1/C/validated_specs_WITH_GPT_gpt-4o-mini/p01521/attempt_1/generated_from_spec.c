#include <stdio.h>

#define MAX_LENGTH 200 // 199 characters + null terminator

int main() {
    char input[MAX_LENGTH]; // Array to hold the input string
    int count_o = 0; // Counter for 'o'
    int count_x = 0; // Counter for 'x'

    // Read the input string
    fgets(input, MAX_LENGTH, stdin);

    // Count occurrences of 'o' and 'x'
    for (int i = 0; input[i] != '\0' && i < MAX_LENGTH; i++) {
        if (input[i] == 'o') {
            count_o++;
        } else if (input[i] == 'x') {
            count_x++;
        }
    }

    // Output 'x' if count_x > count_o, otherwise output 'o'
    // If both counts are equal, 'o' is output as per the specification
    if (count_x > count_o) {
        putchar('x');
    } else {
        putchar('o');
    }

    return 0;
}
