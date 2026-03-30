#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    char character;
    int value;
} Card;

// Function to compare two cards for quicksort
int compare(const void *a, const void *b) {
    Card *cardA = (Card *)a;
    Card *cardB = (Card *)b;
    return cardA->value - cardB->value; // Sort by integer value
}

// Function to check stability of sorting
int isStable(Card *original, Card *sorted, int n) {
    for (int i = 0; i < n; i++) {
        // Check if the original order is preserved for equal values
        for (int j = i + 1; j < n; j++) {
            if (original[i].value == original[j].value) {
                // Find the positions in the sorted array
                int posA = -1, posB = -1;
                for (int k = 0; k < n; k++) {
                    if (sorted[k].character == original[i].character && posA == -1) {
                        posA = k;
                    }
                    if (sorted[k].character == original[j].character && posB == -1) {
                        posB = k;
                    }
                }
                // If the positions are reversed, sorting is not stable
                if (posA > posB) {
                    return 0; // Not stable
                }
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
        scanf(" %c %d", &cards[i].character, &cards[i].value); // Read each card
    }

    Card *original = malloc(n * sizeof(Card)); // Allocate memory for original cards
    memcpy(original, cards, n * sizeof(Card)); // Copy original cards for stability check

    qsort(cards, n, sizeof(Card), compare); // Sort cards using quicksort

    // Check if the sorting is stable
    int stable = isStable(original, cards, n);

    // Output stability result
    printf("%s\n", stable ? "Stable" : "Not stable");

    // Output sorted cards
    for (int i = 0; i < n; i++) {
        printf("%c %d\n", cards[i].character, cards[i].value);
    }

    free(cards); // Free allocated memory
    free(original); // Free original cards memory
    return 0;
}
