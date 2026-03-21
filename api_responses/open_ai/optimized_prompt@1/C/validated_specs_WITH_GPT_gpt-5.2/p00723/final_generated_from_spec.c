#include <stdio.h>
#include <string.h>

#define MAX_LEN 80          /* maximum length strictly less than 80 (excluding '\0') */
#define MAX_UNIQUE 1024     /* fixed storage capacity for unique candidates */

static void reverse_range(char *s, int l, int r) {
    /* reverse s[l..r] inclusive; assumes indices valid */
    while (l < r) {
        char tmp = s[l];
        s[l] = s[r];
        s[r] = tmp;
        l++;
        r--;
    }
}

static int add_unique(char store[MAX_UNIQUE][MAX_LEN], int *count, const char *cand) {
    int i;
    for (i = 0; i < *count; i++) {
        if (strcmp(store[i], cand) == 0) return 0; /* already present */
    }
    /* If capacity exceeded, behavior is unspecified by spec; we simply avoid writing past bounds. */
    if (*count >= MAX_UNIQUE) return 0;
    strncpy(store[*count], cand, MAX_LEN - 1);
    store[*count][MAX_LEN - 1] = '\0';
    (*count)++;
    return 1;
}

static void rotate_prefix_to_end(char *dst, const char *src, int L, int i) {
    /* dst := src[i..L-1] + src[0..i-1], length L, null-terminated */
    int k = 0;
    int j;
    for (j = i; j < L; j++) dst[k++] = src[j];
    for (j = 0; j < i; j++) dst[k++] = src[j];
    dst[L] = '\0';
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    while (n-- > 0) {
        char t[MAX_LEN];
        char s[MAX_LEN];
        char unique[MAX_UNIQUE][MAX_LEN];
        int unique_count = 0;

        if (scanf("%79s", t) != 1) return 0;

        int L = (int)strlen(t);
        if (L <= 1) {
            printf("0\n");
            continue;
        }

        for (int i = 1; i <= L - 1; i++) {
            /* A. Reversal-based sequence on split (i, L-i) */
            strcpy(s, t);
            add_unique(unique, &unique_count, s);

            reverse_range(s, 0, i - 1);
            add_unique(unique, &unique_count, s);

            reverse_range(s, i, L - 1);
            add_unique(unique, &unique_count, s);

            reverse_range(s, 0, i - 1);
            add_unique(unique, &unique_count, s);

            /* B. Rotation-then-reversal sequence */
            rotate_prefix_to_end(s, t, L, i);
            add_unique(unique, &unique_count, s);

            /* reverse prefix length (L-i): indices [0 .. (L-i)-1] */
            reverse_range(s, 0, (L - i) - 1);
            add_unique(unique, &unique_count, s);

            /* reverse suffix length i: indices [L-i .. L-1] */
            reverse_range(s, L - i, L - 1);
            add_unique(unique, &unique_count, s);

            /* reverse prefix length (L-i) again */
            reverse_range(s, 0, (L - i) - 1);
            add_unique(unique, &unique_count, s);
        }

        printf("%d\n", unique_count);
    }

    return 0;
}
