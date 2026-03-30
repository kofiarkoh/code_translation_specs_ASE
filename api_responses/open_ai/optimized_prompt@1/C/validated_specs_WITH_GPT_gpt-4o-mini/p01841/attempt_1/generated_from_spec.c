#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LENGTH 100 // Define a maximum length for input strings

// Function to parse the tree from the input string and return the root value
int parse_tree(const char *tree_str) {
    // If the tree string is empty (length of 2), return 0 (empty tree)
    if (strlen(tree_str) <= 2) {
        return 0; // Represents an empty tree
    }
    
    // Extract the root value (assuming the format is "<value>...")
    // Here we assume the value is an integer followed by a space or a character
    int root_value = 0;
    sscanf(tree_str + 1, "%d", &root_value); // Skip the opening character
    return root_value; // Return the root value
}

// Function to combine two trees by summing their root values
char* combine_trees(const char *tree1, const char *tree2) {
    int root1 = parse_tree(tree1); // Parse the first tree
    int root2 = parse_tree(tree2); // Parse the second tree
    
    // Combine the root values
    int combined_root = root1 + root2;
    
    // Allocate memory for the output string
    char *output = (char *)malloc(MAX_LENGTH * sizeof(char));
    if (output == NULL) {
        return NULL; // Handle memory allocation failure
    }
    
    // Format the output string as "<combined_root>"
    snprintf(output, MAX_LENGTH, "<%d>", combined_root);
    return output; // Return the combined tree as a string
}

int main() {
    char tree1[MAX_LENGTH];
    char tree2[MAX_LENGTH];
    
    // Read two input strings representing the trees
    fgets(tree1, MAX_LENGTH, stdin);
    fgets(tree2, MAX_LENGTH, stdin);
    
    // Remove newline characters from the input strings
    tree1[strcspn(tree1, "\n")] = 0;
    tree2[strcspn(tree2, "\n")] = 0;
    
    // Combine the trees and get the output string
    char *result = combine_trees(tree1, tree2);
    
    // Print the resulting combined tree
    if (result != NULL) {
        printf("%s\n", result);
        free(result); // Free the allocated memory for the output string
    }
    
    return 0; // Exit the program
}
