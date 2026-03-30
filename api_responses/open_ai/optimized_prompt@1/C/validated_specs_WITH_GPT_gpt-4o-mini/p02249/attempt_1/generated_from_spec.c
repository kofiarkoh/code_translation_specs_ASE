#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_HEIGHT 100
#define MAX_WIDTH 100

// Function to compute a simple hash for a grid
unsigned long compute_hash(char grid[MAX_HEIGHT][MAX_WIDTH], int height, int width) {
    unsigned long hash = 0;
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            hash = hash * 31 + grid[i][j]; // Simple polynomial rolling hash
        }
    }
    return hash;
}

// Function to search for the pattern in the text grid
void search_pattern(int text_height, int text_width, char text[MAX_HEIGHT][MAX_WIDTH],
                    int pattern_height, int pattern_width, char pattern[MAX_HEIGHT][MAX_WIDTH]) {
    unsigned long pattern_hash = compute_hash(pattern, pattern_height, pattern_width);
    
    // Iterate over all possible starting positions in the text grid
    for (int i = 0; i <= text_height - pattern_height; i++) {
        for (int j = 0; j <= text_width - pattern_width; j++) {
            // Compute the hash for the current sub-grid of the text
            unsigned long text_hash = compute_hash(&text[i][j], pattern_height, pattern_width);
            
            // Compare hashes
            if (text_hash == pattern_hash) {
                // If hashes match, we can output the starting position
                printf("Match found at: (%d, %d)\n", i, j);
            }
        }
    }
}

int main() {
    // Example usage
    int text_height = 5, text_width = 5;
    char text[MAX_HEIGHT][MAX_WIDTH] = {
        {'a', 'b', 'c', 'd', 'e'},
        {'f', 'g', 'h', 'i', 'j'},
        {'k', 'l', 'm', 'n', 'o'},
        {'p', 'q', 'r', 's', 't'},
        {'u', 'v', 'w', 'x', 'y'}
    };

    int pattern_height = 2, pattern_width = 2;
    char pattern[MAX_HEIGHT][MAX_WIDTH] = {
        {'g', 'h'},
        {'l', 'm'}
    };

    search_pattern(text_height, text_width, text, pattern_height, pattern_width, pattern);
    
    return 0;
}
