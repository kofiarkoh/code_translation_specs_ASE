#include <stdio.h>
#include <string.h>

#define CARD_COUNT 20

// Function to recursively remove pairs of adjacent matching cards
int remove_pairs(char cards[CARD_COUNT + 1]) {
    int i, j;
    int removed = 0;

    // Iterate through the card sequence
    for (i = 0; i < CARD_COUNT - 1; i++) {
        // Check for a pair of matching adjacent cards
        if (cards[i] == cards[i + 1]) {
            // Create a new sequence without the matched pair
            char new_cards[CARD_COUNT + 1];
            strncpy(new_cards, cards, i); // Copy cards before the pair
            strncpy(new_cards + i, cards + i + 2, CARD_COUNT - i - 2); // Copy cards after the pair
            new_cards[CARD_COUNT - 2] = '\0'; // Null-terminate the new string

            // Recursively remove pairs from the new sequence
            removed = 1 + remove_pairs(new_cards); // Count this removal
            break; // Restart the loop after a removal
        }
    }

    // Return the number of pairs removed
    return removed;
}

int main() {
    int n; // Number of test cases
    scanf("%d", &n); // Read the number of test cases

    for (int i = 0; i < n; i++) {
        char cards[CARD_COUNT + 1]; // Buffer for the card sequence
        scanf("%s", cards); // Read the card sequence

        // Count the number of pairs removed
        int pairs_removed = remove_pairs(cards);
        
        // Calculate remaining cards
        int remaining_cards = CARD_COUNT - (pairs_removed * 2);
        
        // Output the number of remaining cards
        printf("%d\n", remaining_cards);
    }

    return 0;
}
