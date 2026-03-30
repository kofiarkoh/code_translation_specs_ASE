#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LENGTH 100000

// Function to parse the tree string and extract the root value and subtrees
// Returns the length of the parsed tree string
int parse_tree(const char *tree_str, int *root_value, char **left_subtree, char **right_subtree) {
    // Assuming the tree format is "value(left_subtree)(right_subtree)"
    // Example: "1(2)(3)" means root value 1, left child 2, right child 3
    if (tree_str == NULL || strlen(tree_str) < 2) {
        return 0; // Invalid tree
    }

    // Find the root value
    int value = 0;
    int i = 0;
    while (tree_str[i] >= '0' && tree_str[i] <= '9') {
        value = value * 10 + (tree_str[i] - '0');
        i++;
    }
    *root_value = value;

    // Skip to the left subtree
    if (tree_str[i] == '(') {
        int start = i + 1; // Start of left subtree
        int depth = 1; // Track depth of parentheses
        while (depth > 0 && tree_str[i] != '\0') {
            i++;
            if (tree_str[i] == '(') depth++;
            else if (tree_str[i] == ')') depth--;
        }
        if (depth == 0) {
            *left_subtree = strndup(tree_str + start, i - start); // Extract left subtree
        }
    }

    // Skip to the right subtree
    if (tree_str[i] == '(') {
        int start = i + 1; // Start of right subtree
        int depth = 1; // Track depth of parentheses
        while (depth > 0 && tree_str[i] != '\0') {
            i++;
            if (tree_str[i] == '(') depth++;
            else if (tree_str[i] == ')') depth--;
        }
        if (depth == 0) {
            *right_subtree = strndup(tree_str + start, i - start); // Extract right subtree
        }
    }

    return 1; // Successfully parsed
}

// Function to combine two trees
void combine_trees(const char *tree1, const char *tree2, char **output_tree) {
    int root1, root2;
    char *left1 = NULL, *right1 = NULL;
    char *left2 = NULL, *right2 = NULL;

    // Parse both trees
    int valid1 = parse_tree(tree1, &root1, &left1, &right1);
    int valid2 = parse_tree(tree2, &root2, &left2, &right2);

    // Handle empty trees
    if (!valid1) {
        *output_tree = strdup("()"); // Empty tree representation
        return;
    }
    if (!valid2) {
        *output_tree = strdup("()"); // Empty tree representation
        return;
    }

    // Combine root values
    int combined_root = root1 + root2;

    // Create output tree structure
    // Format: "combined_root(left_subtree)(right_subtree)"
    int output_length = snprintf(NULL, 0, "%d(%s)(%s)", combined_root, left1 ? left1 : "()", right1 ? right1 : "()") + 1;
    *output_tree = (char *)malloc(output_length);
    snprintf(*output_tree, output_length, "%d(%s)(%s)", combined_root, left1 ? left1 : "()", right1 ? right1 : "()");

    // Free allocated subtrees
    free(left1);
    free(right1);
    free(left2);
    free(right2);
}

int main() {
    char tree1[MAX_LENGTH + 1];
    char tree2[MAX_LENGTH + 1];

    // Read input trees
    if (fgets(tree1, sizeof(tree1), stdin) == NULL || fgets(tree2, sizeof(tree2), stdin) == NULL) {
        return 1; // Error reading input
    }

    // Remove newline characters
    tree1[strcspn(tree1, "\n")] = 0;
    tree2[strcspn(tree2, "\n")] = 0;

    char *output_tree = NULL;
    combine_trees(tree1, tree2, &output_tree);

    // Print the combined tree
    printf("%s\n", output_tree);

    // Free the output tree
    free(output_tree);
    return 0;
}
