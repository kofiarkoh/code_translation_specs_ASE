#include <stdio.h>
#include <stdlib.h>

int calculate_score(const char *sequence, int length, int window_size) {
    int score = 0;
    // Count occurrences of 'D' and 'M' in the first window
    for (int i = 0; i < window_size; i++) {
        if (sequence[i] == 'D') {
            score++;
        } else if (sequence[i] == 'M') {
            score += 2; // Assuming 'M' contributes more to the score
        }
    }
    return score;
}

void process_queries(int length, const char *sequence, int num_queries, const int *window_sizes) {
    for (int i = 0; i < num_queries; i++) {
        int window_size = window_sizes[i];
        if (window_size > 0 && window_size <= length) {
            int score = calculate_score(sequence, length, window_size);
            printf("%d\n", score);
            // Slide the window across the sequence
            for (int j = 1; j <= length - window_size; j++) {
                // Update score based on the character exiting and entering the window
                if (sequence[j - 1] == 'D') {
                    score--;
                } else if (sequence[j - 1] == 'M') {
                    score -= 2; // Assuming 'M' contributes more to the score
                }
                if (sequence[j + window_size - 1] == 'D') {
                    score++;
                } else if (sequence[j + window_size - 1] == 'M') {
                    score += 2; // Assuming 'M' contributes more to the score
                }
                printf("%d\n", score);
            }
        }
    }
}

int main() {
    int length;
    scanf("%d", &length);
    
    char *sequence = (char *)malloc((length + 1) * sizeof(char));
    scanf("%s", sequence);
    
    int num_queries;
    scanf("%d", &num_queries);
    
    int *window_sizes = (int *)malloc(num_queries * sizeof(int));
    for (int i = 0; i < num_queries; i++) {
        scanf("%d", &window_sizes[i]);
    }
    
    process_queries(length, sequence, num_queries, window_sizes);
    
    free(sequence);
    free(window_sizes);
    return 0;
}
