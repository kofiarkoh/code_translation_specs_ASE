#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define MAX_WORDS   1024   /* fixed maximum capacity for distinct words */
#define MAX_WORDLEN 64     /* includes terminator; input must fit (precondition) */
#define MAX_PAGES   1024   /* fixed inclusive range starting at 0: [0, MAX_PAGES-1] */

typedef struct {
    char word[MAX_WORDLEN];
    unsigned char pages[MAX_PAGES]; /* boolean set: 1 if page present */
    unsigned int occ_count;         /* total occurrences read for this word */
} Entry;

static Entry g_entries[MAX_WORDS];
static int g_nentries = 0;

static int find_entry(const char *w) {
    /* Linear search; quadratic overall behavior as per spec. */
    for (int i = 0; i < g_nentries; i++) {
        if (strcmp(g_entries[i].word, w) == 0) return i;
    }
    return -1;
}

static int cmp_entry_word(const void *a, const void *b) {
    const Entry *ea = (const Entry *)a;
    const Entry *eb = (const Entry *)b;
    return strcmp(ea->word, eb->word);
}

int main(void) {
    char wbuf[MAX_WORDLEN];
    int page;

    /* Read records until EOF. Each record: word token + non-negative integer page. */
    while (scanf("%63s %d", wbuf, &page) == 2) {
        /* Preconditions say page is within fixed inclusive range starting at 0.
           If violated, behavior is undefined; we do not guard. */
        int idx = find_entry(wbuf);
        if (idx < 0) {
            /* Preconditions say number of distinct words does not exceed MAX_WORDS.
               If violated, behavior is undefined; we do not guard. */
            idx = g_nentries++;
            /* Initialize new entry */
            strcpy(g_entries[idx].word, wbuf); /* precondition: fits */
            memset(g_entries[idx].pages, 0, sizeof(g_entries[idx].pages));
            g_entries[idx].occ_count = 0;
        }

        /* Record occurrence count (non-decreasing invariant). */
        g_entries[idx].occ_count++;

        /* Add page to set. Preconditions say no repeated (word,page) for a word. */
        g_entries[idx].pages[page] = 1;
    }

    /* Sort entries by word lexicographically ascending. */
    qsort(g_entries, (size_t)g_nentries, sizeof(g_entries[0]), cmp_entry_word);

    /* Emit output: for each word, one line with word, then one line with pages ascending. */
    for (int i = 0; i < g_nentries; i++) {
        puts(g_entries[i].word);

        int first = 1;
        for (int p = 0; p < MAX_PAGES; p++) {
            if (g_entries[i].pages[p]) {
                if (!first) putchar(' ');
                printf("%d", p);
                first = 0;
            }
        }
        putchar('\n');
    }

    return 0;
}
