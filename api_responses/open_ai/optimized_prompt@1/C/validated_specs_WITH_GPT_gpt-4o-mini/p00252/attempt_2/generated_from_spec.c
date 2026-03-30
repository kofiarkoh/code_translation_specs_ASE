#include <stdio.h>

const char* evaluate_state(int a, int b, int c) {
    // Check if inputs are within the expected range (0 or 1)
    // The specification does not handle invalid inputs, so we assume they are valid.
    
    // Determine the output based on the specified input combinations
    if (a == 1 && b == 1 && c == 0) {
        return "Open"; // Input (1, 1, 0)
    } else if (a == 0 && b == 0 && c == 1) {
        return "Open"; // Input (0, 0, 1)
    } else {
        return "Close"; // All other combinations default to "Close"
    }
}

int main() {
    // Example usage of the evaluate_state function
    int a, b, c;
    
    // Assuming inputs are provided correctly as 0 or 1
    // In a real scenario, you would get these values from user input or another source
    a = 1; // Example input
    b = 1; // Example input
    c = 0; // Example input
    
    const char* result = evaluate_state(a, b, c);
    printf("%s\n", result); // Output the result

    return 0;
}
