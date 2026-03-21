#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define PAPER_MAX 64  /* fixed buffer; spec: must fit including NUL */

static void render_partition(const char *paper, int n, unsigned long long mask, char *out, size_t outsz) {
    /* mask bit i (0..n-2) indicates a cut after position i */
    size_t pos = 0;
    for (int i = 0; i < n; i++) {
        if (pos + 1 >= outsz) break;
        out[pos++] = paper[i];
        if (i < n - 1 && ((mask >> i) & 1ULL)) {
            if (pos + 1 >= outsz) break;
            out[pos++] = ' ';
        }
    }
    if (outsz > 0) {
        if (pos >= outsz) pos = outsz - 1;
        out[pos] = '\0';
    }
}

int main(void) {
    long target;
    char paper[PAPER_MAX];

    while (scanf("%ld %63s", &target, paper) == 2) {
        if (target == 0)
            break;

        int n = (int)strlen(paper);
        /* Number of partitions is 2^(n-1). Use 64-bit mask; if n-1 >= 64, behavior is unspecified by spec. */
        unsigned long long total_masks = 1ULL;
        if (n > 1) total_masks <<= (unsigned)(n - 1);

        long best = 0;
        int have_best = 0;
        int best_count = 0;
        unsigned long long best_mask = 0;

        for (unsigned long long mask = 0; mask < total_masks; mask++) {
            long sum = 0;

            int start = 0;
            for (int i = 0; i < n; i++) {
                int cut = (i == n - 1) || ((mask >> i) & 1ULL);
                if (cut) {
                    char tmp[PAPER_MAX];
                    int len = i - start + 1;
                    if (len >= (int)sizeof(tmp)) {
                        /* Should not happen given PAPER_MAX and n, but keep narrow behavior. */
                        len = (int)sizeof(tmp) - 1;
                    }
                    memcpy(tmp, paper + start, (size_t)len);
                    tmp[len] = '\0';

                    /* Standard decimal parsing; leading zeros allowed; non-digits follow strtol behavior. */
                    long val = strtol(tmp, NULL, 10);
                    sum += val;

                    start = i + 1;
                }
            }

            if (sum <= target) {
                if (!have_best || sum > best) {
                    have_best = 1;
                    best = sum;
                    best_count = 1;
                    best_mask = mask;
                } else if (sum == best) {
                    best_count++;
                }
            }
        }

        if (!have_best) {
            puts("error");
        } else if (best_count != 1) {
            puts("rejected");
        } else {
            char rendered[PAPER_MAX * 2]; /* enough for digits + spaces */
            render_partition(paper, n, best_mask, rendered, sizeof(rendered));
            printf("%ld %s\n", best, rendered);
        }
    }

    return 0;
}
