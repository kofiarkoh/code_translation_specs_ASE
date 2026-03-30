#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MD 998244353
#define MAX_LENGTH 4000

// Function to convert a binary string to an integer
unsigned long long binary_to_int(const char *binary) {
    unsigned long long result = 0;
    while (*binary) {
        result = (result << 1) | (*binary - '0');
        binary++;
    }
    return result;
}

int main() {
    int m;
    char primary_string[MAX_LENGTH + 1];
    
    // Read the number of additional binary strings
    scanf("%d", &m);
    
    // Read the primary binary string
    scanf("%s", primary_string);
    
    // Convert the primary binary string to an integer
    unsigned long long primary_value = binary_to_int(primary_string);
    unsigned long long result = primary_value; // Initialize result with primary value

    // Process each additional binary string
    for (int i = 0; i < m; i++) {
        char additional_string[MAX_LENGTH + 1];
        scanf("%s", additional_string);
        
        // Convert the additional binary string to an integer
        unsigned long long additional_value = binary_to_int(additional_string);
        
        // Perform bitwise operations based on the relative positions of bits
        // Here we assume a simple XOR operation for demonstration
        result ^= additional_value; // XOR with the additional value
    }

    // Ensure the result is within the defined range
    result %= MD;

    // Print the final result
    printf("%llu\n", result);
    
    return 0;
}
