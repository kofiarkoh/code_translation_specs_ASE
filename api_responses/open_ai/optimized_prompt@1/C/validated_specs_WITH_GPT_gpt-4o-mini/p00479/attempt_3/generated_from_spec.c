#include <stdio.h>

int main() {
    int n, k;
    
    // Read the total range and number of pairs
    scanf("%d %d", &n, &k);
    
    // Process each pair
    for (int i = 0; i < k; i++) {
        int ai, bi;
        scanf("%d %d", &ai, &bi);
        
        // Adjust ai if necessary
        if (n + 1 - ai < ai) {
            ai = n + 1 - ai; // Adjust ai to n + 1 - ai if it's greater
        }
        
        // Adjust bi if necessary
        if (n + 1 - bi < bi) {
            bi = n + 1 - bi; // Adjust bi to n + 1 - bi if it's greater
        }
        
        // Compute the output based on the adjusted values
        int output;
        if (ai > bi) {
            output = (bi - 1) % 3 + 1; // Output based on bi
        } else {
            output = (ai - 1) % 3 + 1; // Output based on ai
        }
        
        // Print the output for the current pair
        printf("%d\n", output);
    }
    
    return 0;
}
