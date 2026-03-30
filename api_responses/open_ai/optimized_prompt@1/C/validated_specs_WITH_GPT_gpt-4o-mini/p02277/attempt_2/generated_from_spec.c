#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    char character;
    int value;
    int original_index; // To check stability
} Card;

// Comparison function for quicksort
int compare(const void *a, const void *b) {
    Card *cardA = (Card *)a;
    Card *cardB = (Card *)b;
    if (cardA->value == cardB->value) {
        return cardA->original_index - cardB->original_index; // Preserve original order for stability
    }
    return cardA->value - cardB->value;
}

// Function to check stability
int is_stable(Card *cards, int n) {
    for (int i = 0; i < n - 1; i++) {
        if (cards[i].value == cards[i + 1].value) {
            if (cards[i].original_index > cards[i + 1].original_index) {
                return 0; // Not stable
            }
        }
    }
    return 1; // Stable
}

int main() {
    int n;
    scanf("%d", &n); // Read number of cards

    Card *cards = malloc(n * sizeof(Card)); // Allocate memory for cards
    for (int i = 0; i < n; i++) {
        scanf(" %c %d", &cards[i].character, &cards[i].value); // Read character and integer
        cards[i].original_index = i; // Store original index
    }

    qsort(cards, n, sizeof(Card), compare); // Sort cards using quicksort

    // Check stability
    if (is_stable(cards, n)) {
        printf("Stable\n");
    } else {
        printf("Not stable\n");
    }

    // Print sorted cards
    for (int i = 0; i < n; i++) {
        printf("%c %d\n", cards[i].character, cards[i].value);
    }

    free(cards); // Free allocated memory
    return 0;
}
