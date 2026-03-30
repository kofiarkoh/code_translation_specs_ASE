#include <stdio.h>

const char* evaluate_state(int a, int b, int c) {
    // Evaluate the state based on the specific combinations of input integers
    if ((a == 1 && b == 0 && c == 0) || 
        (a == 0 && b == 1 && c == 0) || 
        (a == 0 && b == 0 && c == 0)) {
        return "Close"; // Outputs "Close" for specified combinations
    } else if ((a == 1 && b == 1 && c == 0) || 
               (a == 0 && b == 0 && c == 1)) {
        return "Open"; // Outputs "Open" for specified combinations
    }
    // The specification does not define behavior for other combinations,
    // but we will return "Close" as a safe default.
    return "Close"; // Default case to maintain consistent output
}

int main() {
    int a, b, c;
    // Example input reading (not specified in the task, but necessary for testing)
    printf("Enter three integers: ");
    scanf("%d %d %d", &a, &b, &c);
    
    // Output the evaluated state
    printf("%s\n", evaluate_state(a, b, c));
    return 0;
}
