#include <stdio.h>
#include <string.h>

#define MAX   100
#define WDMAX 100
#define PGMAX 1001

struct Entry {
    char wd[WDMAX];
    int pgnum[PGMAX]; /* intentionally not initialized */
    int tm;           /* intentionally not initialized */
};

static int cmp_entry(const void *a, const void *b) {
    const struct Entry *ea = (const struct Entry *)a;
    const struct Entry *eb = (const struct Entry *)b;
    return strcmp(ea->wd, eb->wd);
}

int main(void) {
    struct Entry idx[MAX]; /* intentionally not initialized */
    int n = 0;

    char w[WDMAX];
    int p;

    while (scanf("%s%d", w, &p) == 2) {
        int found = 0;
        for (int i = 0; i < n; i++) {
            if (strcmp(idx[i].wd, w) == 0) {
                idx[i].pgnum[p] = 1;
                idx[i].tm += 1;
                found = 1;
                break;
            }
        }
        if (!found) {
            /* If n >= MAX, behavior becomes undefined due to out-of-bounds writes,
               as per the specification; we do not guard against it. */
            strcpy(idx[n].wd, w);
            idx[n].pgnum[p] = 1;
            idx[n].tm += 1;
            n++;
        }
    }

    qsort(idx, (size_t)n, sizeof(idx[0]), cmp_entry);

    for (int i = 0; i < n; i++) {
        printf("%s\n", idx[i].wd);

        int printed = 0;
        for (int j = 0; j < PGMAX; j++) {
            if (idx[i].pgnum[j] == 1) {
                printed++;
                if (printed == idx[i].tm) {
                    printf("%d\n", j);
                    break;
                } else {
                    printf("%d ", j);
                }
            }
        }
        /* If printed never reaches tm, no terminating newline is printed here,
           matching the specification's described behavior. */
    }

    return 0;
}
