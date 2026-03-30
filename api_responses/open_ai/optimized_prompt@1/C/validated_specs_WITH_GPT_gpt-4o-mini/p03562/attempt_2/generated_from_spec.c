#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MD 998244353
#define MAX_LENGTH 4000

// Function to convert a binary string to an integer
unsigned long long binary_to_int(const char *binary) {
    unsigned long long result = 0;
    while (*binary) {
        result = (result << 1) | (*binary - '0'); // Shift left and add the next bit
        binary++;
    }
    return result;
}

int main() {
    int m;
    char s[MAX_LENGTH + 1]; // Buffer for the main binary string
    unsigned long long result = 0;

    // Read the number of binary strings
    if (scanf("%d", &m) != 1) {
        return 1; // Error reading m
    }

    // Read the main binary string
    if (scanf("%s", s) != 1) {
        return 1; // Error reading binary string
    }

    // Convert the main binary string to an integer
    unsigned long long base_value = binary_to_int(s);

    // Process each additional binary string
    for (int i = 0; i < m; i++) {
        char temp[MAX_LENGTH + 1]; // Buffer for additional binary strings
        if (scanf("%s", temp) != 1) {
            return 1; // Error reading additional binary string
        }

        // Convert the additional binary string to an integer
        unsigned long long additional_value = binary_to_int(temp);

        // Perform bitwise operations based on the index
        // Here we assume a simple XOR operation for demonstration
        result ^= (base_value + (additional_value << i)); // Shift and XOR
    }

    // Ensure the result is within the defined range
    result %= MD;

    // Print the final result
    printf("%llu\n", result);
    return 0;
}
