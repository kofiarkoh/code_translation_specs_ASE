#include <stdio.h>

#define MAX_SIZE 100

int main() {
    int n; // Number of elements in the first list
    int list[MAX_SIZE]; // The list of integers
    int m; // Number of modifications
    int index, new_value; // Variables for modification pairs
    int modified_sum; // To store the modified sum

    // Read the number of elements in the list
    scanf("%d", &n);
    
    // Read the elements of the list
    for (int i = 0; i < n; i++) {
        scanf("%d", &list[i]);
    }

    // Calculate the initial sum of the list
    int initial_sum = 0;
    for (int i = 0; i < n; i++) {
        initial_sum += list[i];
    }

    // Read the number of modifications
    scanf("%d", &m);

    // Process each modification
    for (int j = 0; j < m; j++) {
        // Read the index and new value
        scanf("%d %d", &index, &new_value);
        
        // Update the list at the specified index
        // The behavior is undefined if index is invalid, so no checks are performed
        initial_sum = initial_sum - list[index] + new_value; // Update the sum
        list[index] = new_value; // Modify the list
        
        // Output the modified sum
        printf("%d\n", initial_sum);
    }

    return 0;
}
