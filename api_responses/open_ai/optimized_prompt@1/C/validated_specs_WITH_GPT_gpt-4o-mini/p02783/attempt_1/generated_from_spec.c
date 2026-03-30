#include <stdio.h>

int main() {
    int dividend, divisor;
    
    // Read two integers from input
    scanf("%d %d", &dividend, &divisor);
    
    // The divisor must not be zero; however, we do not handle this case.
    // If divisor is zero, the behavior is undefined as per the specification.
    
    // Compute the ceiling of the division
    int result;
    if (dividend % divisor == 0) {
        // If the division is exact, the result is simply dividend / divisor
        result = dividend / divisor;
    } else {
        // If not exact, we need to compute the ceiling
        result = (dividend / divisor) + 1; // Add 1 to the quotient
    }
    
    // Output the result
    printf("%d\n", result);
    
    return 0;
}
