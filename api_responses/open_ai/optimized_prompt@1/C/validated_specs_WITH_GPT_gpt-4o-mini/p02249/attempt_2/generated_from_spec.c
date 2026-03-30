#include <stdio.h>
#include <string.h>

#define MAX_H 100 // Assuming a maximum size for the text grid
#define MAX_W 100 // Assuming a maximum size for the text grid
#define MAX_R 10  // Assuming a maximum size for the pattern grid
#define MAX_C 10  // Assuming a maximum size for the pattern grid

// Function to compute a simple hash for a grid
unsigned long compute_hash(char grid[MAX_H][MAX_W], int height, int width) {
    unsigned long hash = 0;
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            hash = hash * 31 + grid[i][j]; // Using a prime number for hashing
        }
    }
    return hash;
}

// Function to search for the pattern in the text grid
void search_pattern(int H, int W, char text[MAX_H][MAX_W], int R, int C, char pattern[MAX_R][MAX_C]) {
    unsigned long pattern_hash = compute_hash(pattern, R, C);
    
    for (int i = 0; i <= H - R; i++) {
        for (int j = 0; j <= W - C; j++) {
            // Compute hash for the current sub-grid in the text
            unsigned long text_hash = compute_hash(&text[i][j], R, C);
            
            // Only compare if hashes match
            if (text_hash == pattern_hash) {
                // Check for exact character match
                int match = 1;
                for (int r = 0; r < R; r++) {
                    for (int c = 0; c < C; c++) {
                        if (text[i + r][j + c] != pattern[r][c]) {
                            match = 0; // Found a mismatch
                            break;
                        }
                    }
                    if (!match) break; // Exit outer loop if mismatch found
                }
                
                // If a match is found, output the starting position
                if (match) {
                    printf("Match found at: (%d, %d)\n", i, j);
                }
            }
        }
    }
}

int main() {
    // Example usage
    int H = 5, W = 5; // Dimensions of the text grid
    char text[MAX_H][MAX_W] = {
        {'a', 'b', 'c', 'd', 'e'},
        {'f', 'g', 'h', 'i', 'j'},
        {'k', 'l', 'm', 'n', 'o'},
        {'p', 'q', 'r', 's', 't'},
        {'u', 'v', 'w', 'x', 'y'}
    };
    
    int R = 2, C = 2; // Dimensions of the pattern grid
    char pattern[MAX_R][MAX_C] = {
        {'g', 'h'},
        {'l', 'm'}
    };
    
    search_pattern(H, W, text, R, C, pattern);
    
    return 0;
}
