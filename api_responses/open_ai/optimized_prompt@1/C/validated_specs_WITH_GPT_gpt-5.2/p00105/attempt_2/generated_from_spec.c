#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define MAX_WORDS 1000
#define WORD_LEN  32
#define MAX_PAGES 1000

struct Entry {
    char word[WORD_LEN];
    int page[MAX_PAGES]; /* intentionally not initialized per spec */
    int tm;              /* intentionally not initialized per spec */
};

static struct Entry entries[MAX_WORDS]; /* static storage => zero-initialized in C */

static int cmp_entries(const void *a, const void *b) {
    const struct Entry *ea = (const struct Entry *)a;
    const struct Entry *eb = (const struct Entry *)b;
    return strcmp(ea->word, eb->word);
}

int main(void) {
    char wbuf[WORD_LEN];
    int p;
    int n = 0;

    while (scanf("%31s%d", wbuf, &p) == 2) {
        int i;
        int found = -1;

        for (i = 0; i < n; i++) {
            if (strcmp(entries[i].word, wbuf) == 0) {
                found = i;
                break;
            }
        }

        if (found >= 0) {
            /* Undefined behavior if p is out of range; we do not guard it. */
            entries[found].page[p] = 1;
            entries[found].tm += 1;
        } else {
            /* Undefined behavior if n exceeds MAX_WORDS; we do not guard it. */
            strcpy(entries[n].word, wbuf); /* undefined if token longer than WORD_LEN-1 */
            entries[n].page[p] = 1;        /* page[] intentionally not initialized */
            entries[n].tm += 1;            /* tm intentionally not initialized */
            n++;
        }
    }

    qsort(entries, (size_t)n, sizeof(entries[0]), cmp_entries);

    for (int i = 0; i < n; i++) {
        printf("%s\n", entries[i].word);

        int printed = 0;
        for (int pg = 0; pg < MAX_PAGES; pg++) {
            if (entries[i].page[pg] == 1) {
                printed++;
                if (printed == entries[i].tm) {
                    printf("%d\n", pg);
                    break;
                } else {
                    printf("%d ", pg);
                }
            }
        }
        /* If tm is larger than the number of pages with presence==1, no newline is printed. */
    }

    return 0;
}
