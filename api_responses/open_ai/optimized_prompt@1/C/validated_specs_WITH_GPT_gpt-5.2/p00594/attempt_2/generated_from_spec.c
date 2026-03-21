#include <stdio.h>
#include <stdlib.h>

#define MAXN 1000000

static int cmp_desc_sub(const void *a, const void *b) {
    /* Descending order using subtraction, as specified.
       If subtraction overflows, behavior (ordering) is unspecified. */
    int x = *(const int *)a;
    int y = *(const int *)b;
    return y - x;
}

int main(void) {
    static int arr[MAXN];
    int n;

    while (scanf("%d", &n) == 1) {
        if (n == 0) {
            break;
        }

        /* Specification assumes 1..1,000,000 and exactly n integers follow.
           No explicit error handling for malformed input. */
        for (int i = 0; i < n; i++) {
            (void)scanf("%d", &arr[i]);
        }

        qsort(arr, (size_t)n, sizeof(arr[0]), cmp_desc_sub);

        int best_val = arr[0];
        int best_len = 1;

        int cur_val = arr[0];
        int cur_len = 1;

        for (int i = 1; i < n; i++) {
            if (arr[i] == cur_val) {
                cur_len++;
            } else {
                if (cur_len > best_len) {
                    best_len = cur_len;
                    best_val = cur_val;
                }
                cur_val = arr[i];
                cur_len = 1;
            }
        }
        /* Finalize last run */
        if (cur_len > best_len) {
            best_len = cur_len;
            best_val = cur_val;
        }

        if (best_len > n / 2) {
            printf("%d\n", best_val);
        } else {
            printf("NO COLOR\n");
        }
    }

    return 0;
}
