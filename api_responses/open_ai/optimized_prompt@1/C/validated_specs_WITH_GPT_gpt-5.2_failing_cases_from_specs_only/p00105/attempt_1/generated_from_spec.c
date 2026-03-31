#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define MAX_WORDS 100
#define MAX_WORD_LEN 30
#define MAX_PAGE 1000

typedef struct {
    char word[MAX_WORD_LEN + 1];
    /* Set semantics for pages 0..1000 inclusive */
    unsigned char has_page[MAX_PAGE + 1];
    /* Total records seen for this word (not used for output, but tracked per spec) */
    int record_count;
} Entry;

static int cmp_entry_word(const void *a, const void *b) {
    const Entry *ea = (const Entry *)a;
    const Entry *eb = (const Entry *)b;
    return strcmp(ea->word, eb->word);
}

int main(void) {
    Entry entries[MAX_WORDS];
    int n_entries = 0;

    /* Initialize entries array defensively (not required by spec, but safe). */
    for (int i = 0; i < MAX_WORDS; i++) {
        entries[i].word[0] = '\0';
        memset(entries[i].has_page, 0, sizeof(entries[i].has_page));
        entries[i].record_count = 0;
    }

    /* Input: repeated pairs (word, page) until EOF. */
    for (;;) {
        char wbuf[MAX_WORD_LEN + 1];
        int page;

        /* Word token bounded length per precondition; if violated, behavior undefined. */
        if (scanf("%30s", wbuf) != 1) break;
        if (scanf("%d", &page) != 1) break;

        /* Find existing entry. */
        int idx = -1;
        for (int i = 0; i < n_entries; i++) {
            if (strcmp(entries[i].word, wbuf) == 0) {
                idx = i;
                break;
            }
        }

        /* Create new entry if needed. */
        if (idx < 0) {
            idx = n_entries;
            /* If too many distinct words, precondition violated => undefined behavior.
               We still write within bounds only if possible; otherwise, we stop. */
            if (idx >= MAX_WORDS) {
                /* Undefined per spec; choose narrow behavior: stop consuming. */
                break;
            }
            strcpy(entries[idx].word, wbuf);
            memset(entries[idx].has_page, 0, sizeof(entries[idx].has_page));
            entries[idx].record_count = 0;
            n_entries++;
        }

        /* Track record count (precondition bounds it). */
        entries[idx].record_count++;

        /* Record page in set (precondition ensures 0..1000). */
        if (page >= 0 && page <= MAX_PAGE) {
            entries[idx].has_page[page] = 1;
        } else {
            /* Precondition violated => undefined behavior; ignore safely. */
        }
    }

    /* Sort entries by ascending lexicographic order of word. */
    qsort(entries, (size_t)n_entries, sizeof(entries[0]), cmp_entry_word);

    /* Output: for each word, print word line, then pages line. */
    for (int i = 0; i < n_entries; i++) {
        puts(entries[i].word);

        int first = 1;
        for (int p = 0; p <= MAX_PAGE; p++) {
            if (entries[i].has_page[p]) {
                if (!first) putchar(' ');
                printf("%d", p);
                first = 0;
            }
        }
        putchar('\n');
    }

    return 0;
}
