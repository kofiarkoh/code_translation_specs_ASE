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
            ai = n + 1 - ai;
        }
        
        // Adjust bi if necessary
        if (n + 1 - bi < bi) {
            bi = n + 1 - bi;
        }
        
        // Output the minimum of the adjusted values
        // The output is guaranteed to be between 1 and 3 inclusive
        printf("%d\n", (ai < bi) ? ai : bi);
    }
    
    return 0;
}
