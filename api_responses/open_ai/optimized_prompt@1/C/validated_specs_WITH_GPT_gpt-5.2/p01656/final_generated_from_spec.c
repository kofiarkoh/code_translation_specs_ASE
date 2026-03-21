#include <stdio.h>
#include <string.h>

int main(void) {
    int n, query_year;
    if (scanf("%d %d", &n, &query_year) != 2) {
        /* Undefined behavior per spec if input is missing/malformed; exit quietly. */
        return 0;
    }

    /* Fixed, bounded storage: up to 49 records, names up to 31 chars (+ terminator). */
    char years_names[49][32];
    int years[49];

    /* Default name: not specified; choose narrowest reasonable fixed token. */
    char selected[32];
    strcpy(selected, "Unknown");

    for (int i = 0; i < n; i++) {
        int y;
        char name[32];

        if (scanf("%d %31s", &y, name) != 2) {
            /* Undefined behavior per spec; stop. */
            return 0;
        }

        /* Store if within fixed capacity; otherwise behavior is undefined by spec. */
        if (i < 49) {
            years[i] = y;
            strcpy(years_names[i], name);
        }

        if (y <= query_year) {
            /* Update to the most recently read qualifying record. */
            strcpy(selected, name);
        }
    }

    printf("%s\n", selected);
    return 0;
}
