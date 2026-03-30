#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STRINGS 100 // Maximum number of unique strings
#define MAX_PAGES 100   // Maximum number of page numbers per string
#define MAX_STRING_LENGTH 50 // Maximum length of each string

typedef struct {
    char string[MAX_STRING_LENGTH];
    int pages[MAX_PAGES];
    int count; // Count of occurrences (not used in output but maintained)
    int page_count; // Number of page numbers recorded
} StringEntry;

StringEntry entries[MAX_STRINGS];
int unique_count = 0; // Number of unique strings

// Function to find the index of a string in the entries
int find_string_index(const char *str) {
    for (int i = 0; i < unique_count; i++) {
        if (strcmp(entries[i].string, str) == 0) {
            return i;
        }
    }
    return -1; // Not found
}

// Function to compare two StringEntry structures for qsort
int compare_entries(const void *a, const void *b) {
    return strcmp(((StringEntry *)a)->string, ((StringEntry *)b)->string);
}

int main() {
    char input_string[MAX_STRING_LENGTH];
    int page_number;

    // Read input until EOF
    while (scanf("%s %d", input_string, &page_number) == 2) {
        int index = find_string_index(input_string);
        if (index == -1) {
            // New unique string
            if (unique_count < MAX_STRINGS) {
                strcpy(entries[unique_count].string, input_string);
                entries[unique_count].pages[0] = page_number;
                entries[unique_count].page_count = 1;
                entries[unique_count].count = 1; // Increment count for new string
                unique_count++;
            }
        } else {
            // Existing string, add page number
            if (entries[index].page_count < MAX_PAGES) {
                entries[index].pages[entries[index].page_count] = page_number;
                entries[index].page_count++;
                entries[index].count++; // Increment count for existing string
            }
        }
    }

    // Sort the entries based on the string
    qsort(entries, unique_count, sizeof(StringEntry), compare_entries);

    // Output the sorted strings and their associated page numbers
    for (int i = 0; i < unique_count; i++) {
        printf("%s", entries[i].string);
        for (int j = 0; j < entries[i].page_count; j++) {
            printf(" %d", entries[i].pages[j]);
        }
        printf("\n");
    }

    return 0;
}
