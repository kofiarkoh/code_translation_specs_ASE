#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    char character;
    int value;
    int original_index; // To track the original index for stability check
} Card;

// Comparison function for quicksort
int compare(const void *a, const void *b) {
    Card *cardA = (Card *)a;
    Card *cardB = (Card *)b;
    
    // Compare by value first
    if (cardA->value != cardB->value) {
        return cardA->value - cardB->value;
    }
    // If values are equal, maintain original order (stable sort)
    return cardA->original_index - cardB->original_index;
}

// Function to check stability of sorting
const char* check_stability(Card *cards, int n) {
    for (int i = 1; i < n; i++) {
        if (cards[i].value == cards[i - 1].value) {
            // If two cards have the same value, check their original indices
            if (cards[i].original_index < cards[i - 1].original_index) {
                return "Not stable";
            }
        }
    }
    return "Stable";
}

int main() {
    int n;
    scanf("%d", &n); // Read the number of cards

    // Allocate memory for cards
    Card *cards = malloc(n * sizeof(Card));
    
    // Read the cards
    for (int i = 0; i < n; i++) {
        scanf(" %c %d", &cards[i].character, &cards[i].value);
        cards[i].original_index = i; // Store the original index
    }

    // Sort the cards using quicksort
    qsort(cards, n, sizeof(Card), compare);

    // Check stability of the sorting
    const char* stability = check_stability(cards, n);
    
    // Output the stability result
    printf("%s\n", stability);
    
    // Output the sorted cards
    for (int i = 0; i < n; i++) {
        printf("%c %d\n", cards[i].character, cards[i].value);
    }

    // Free allocated memory
    free(cards);
    return 0;
}
