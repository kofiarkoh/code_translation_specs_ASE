#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STRINGS 100 // Maximum number of unique strings
#define MAX_PAGES 100   // Maximum number of page numbers per string

typedef struct {
    char *word;                // Unique string
    int pages[MAX_PAGES];      // Associated page numbers
    int page_count;            // Count of associated page numbers
} StringEntry;

StringEntry entries[MAX_STRINGS]; // Collection of unique strings
int entry_count = 0;               // Current count of unique strings

// Function to compare two StringEntry objects for qsort
int compare(const void *a, const void *b) {
    return strcmp(((StringEntry *)a)->word, ((StringEntry *)b)->word);
}

// Function to find a string in the collection
int find_string(const char *word) {
    for (int i = 0; i < entry_count; i++) {
        if (strcmp(entries[i].word, word) == 0) {
            return i; // Return index if found
        }
    }
    return -1; // Not found
}

int main() {
    char word[256]; // Buffer for input strings
    int page;      // Buffer for input page numbers

    // Read input until EOF
    while (scanf("%255s %d", word, &page) == 2) {
        int index = find_string(word);
        if (index == -1) { // New unique string
            if (entry_count < MAX_STRINGS) {
                entries[entry_count].word = strdup(word); // Allocate memory for the string
                entries[entry_count].pages[0] = page; // Add the first page number
                entries[entry_count].page_count = 1; // Initialize page count
                entry_count++;
            }
        } else { // Existing string
            if (entries[index].page_count < MAX_PAGES) {
                entries[index].pages[entries[index].page_count] = page; // Add page number
                entries[index].page_count++; // Increment page count
            }
        }
    }

    // Sort the unique strings
    qsort(entries, entry_count, sizeof(StringEntry), compare);

    // Output the sorted strings and their associated page numbers
    for (int i = 0; i < entry_count; i++) {
        printf("%s", entries[i].word); // Print the unique string
        for (int j = 0; j < entries[i].page_count; j++) {
            printf(" %d", entries[i].pages[j]); // Print associated page numbers
        }
        printf("\n"); // New line after each entry
        free(entries[i].word); // Free allocated memory for the string
    }

    return 0; // Successful execution
}
