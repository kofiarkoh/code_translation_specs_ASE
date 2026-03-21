#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <inttypes.h>

static int cmp_i64(const void *a, const void *b) {
    int64_t x = *(const int64_t *)a;
    int64_t y = *(const int64_t *)b;
    if (x < y) return -1;
    if (x > y) return 1;
    return 0;
}

typedef struct {
    int64_t x, y;
} Op;

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    int64_t *a = (int64_t *)malloc((size_t)n * sizeof(int64_t));
    if (!a) return 0;

    int has_neg = 0, has_nonneg = 0;
    for (int i = 0; i < n; i++) {
        if (scanf("%" SCNd64, &a[i]) != 1) {
            free(a);
            return 0;
        }
        if (a[i] < 0) has_neg = 1;
        else has_nonneg = 1;
    }

    qsort(a, (size_t)n, sizeof(int64_t), cmp_i64);

    Op *ops = (Op *)malloc((size_t)(n - 1) * sizeof(Op));
    if (!ops) {
        free(a);
        return 0;
    }
    int opcnt = 0;

    /* Construct an operation sequence that attains the maximum described in the spec.
       This is the standard constructive proof:
       - If both negative and non-negative exist: result = sum |ai|
       - If all non-negative: result = sum |ai| - 2*min(ai)
       - If all non-positive: result = sum |ai| + 2*max(ai)
    */

    int64_t result = 0;

    if (has_neg && has_nonneg) {
        /* Mixed signs: use smallest negative as accumulator for non-negatives,
           and largest non-negative as accumulator for remaining negatives. */
        int first_nonneg = 0;
        while (first_nonneg < n && a[first_nonneg] < 0) first_nonneg++;

        /* Preconditions imply both parts non-empty here. */
        int64_t neg_acc = a[0];          /* most negative */
        int64_t pos_acc = a[n - 1];      /* largest non-negative */

        /* Subtract all non-negative except the largest from neg_acc: neg_acc -= ai */
        for (int i = first_nonneg; i <= n - 2; i++) {
            ops[opcnt++] = (Op){neg_acc, a[i]};
            neg_acc = neg_acc - a[i];
        }

        /* Subtract all remaining negatives (except a[0]) from pos_acc: pos_acc -= ai */
        for (int i = 1; i <= first_nonneg - 1; i++) {
            ops[opcnt++] = (Op){pos_acc, a[i]};
            pos_acc = pos_acc - a[i];
        }

        /* Final: pos_acc - neg_acc */
        ops[opcnt++] = (Op){pos_acc, neg_acc};
        result = pos_acc - neg_acc;
    } else if (has_nonneg) {
        /* All non-negative: result = sum(ai) - 2*min(ai).
           Construction:
           - Let neg_acc = min element (a[0]) (non-negative).
           - Subtract all other elements except the maximum from neg_acc: neg_acc -= ai (becomes <= 0).
           - Let pos_acc = maximum element (a[n-1]).
           - Final: pos_acc - neg_acc.
        */
        int64_t neg_acc = a[0];
        int64_t pos_acc = a[n - 1];

        for (int i = 1; i <= n - 2; i++) {
            ops[opcnt++] = (Op){neg_acc, a[i]};
            neg_acc = neg_acc - a[i];
        }

        ops[opcnt++] = (Op){pos_acc, neg_acc};
        result = pos_acc - neg_acc;
    } else {
        /* All non-positive: result = sum(|ai|) + 2*max(ai) where max(ai) <= 0.
           Construction (symmetric to all-non-negative case):
           - Let pos_acc = max element (a[n-1]) (closest to zero).
           - Subtract all other elements except the minimum from pos_acc: pos_acc -= ai (becomes >= 0).
           - Let neg_acc = minimum element (a[0]) (most negative).
           - Final: pos_acc - neg_acc.
        */
        int64_t pos_acc = a[n - 1];
        int64_t neg_acc = a[0];

        for (int i = n - 2; i >= 1; i--) {
            ops[opcnt++] = (Op){pos_acc, a[i]};
            pos_acc = pos_acc - a[i];
        }

        ops[opcnt++] = (Op){pos_acc, neg_acc};
        result = pos_acc - neg_acc;
    }

    /* As per spec, print maximum then exactly n-1 operations. */
    printf("%" PRId64 "\n", result);
    for (int i = 0; i < opcnt; i++) {
        printf("%" PRId64 " %" PRId64 "\n", ops[i].x, ops[i].y);
    }

    /* If preconditions hold, opcnt must be exactly n-1. */
    free(ops);
    free(a);
    return 0;
}
