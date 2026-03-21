#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *data;
    int size;
    int cap;
} Vec;

static void vec_push(Vec *v, int x) {
    if (v->size == v->cap) {
        int newcap = (v->cap == 0) ? 1 : (v->cap * 2);
        int *nd = (int *)realloc(v->data, (size_t)newcap * sizeof(int));
        /* Specification: if allocation fails, behavior is undefined. */
        v->data = nd;
        v->cap = newcap;
    }
    v->data[v->size++] = x;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) {
        /* Specification: undefined if input missing; just exit. */
        return 0;
    }

    /* 1-based indexing as per specification. */
    int *p = (int *)malloc((size_t)(n + 1) * sizeof(int));
    int *ans = (int *)calloc((size_t)(n + 1), sizeof(int));
    Vec *occ = (Vec *)calloc((size_t)(n + 1), sizeof(Vec));

    /* Read p[1..n] and store occurrence positions for each value. */
    for (int i = 1; i <= n; i++) {
        if (scanf("%d", &p[i]) != 1) {
            return 0;
        }
        /* Precondition: 1 <= p[i] <= n; otherwise undefined (out-of-range). */
        vec_push(&occ[p[i]], i);
    }

    /* Stack of "available" positions: extra occurrences beyond the first for each value. */
    int *avail = (int *)malloc((size_t)(n + 1) * sizeof(int));
    int top = 0;

    /* First pass: assign one occurrence to each value that appears; collect extras. */
    for (int v = 1; v <= n; v++) {
        if (occ[v].size > 0) {
            /* Guarantee: if v appears, ans[v] is one of its occurrence positions. */
            ans[v] = occ[v].data[0];
            /* Remaining occurrences become available for missing values. */
            for (int k = 1; k < occ[v].size; k++) {
                avail[top++] = occ[v].data[k];
            }
        } else {
            ans[v] = 0;
        }
    }

    /* Second pass: for values that did not appear, assign any remaining available position. */
    for (int v = 1; v <= n; v++) {
        if (ans[v] == 0) {
            if (top > 0) {
                ans[v] = avail[--top];
                /* For missing v, no guarantee that p[ans[v]] == v (indeed it won't). */
            } else {
                ans[v] = 0;
            }
        }
    }

    /* Output ans[1..n], one per line. */
    for (int v = 1; v <= n; v++) {
        printf("%d\n", ans[v]);
    }

    /* Cleanup (not required by spec, but safe). */
    for (int v = 1; v <= n; v++) {
        free(occ[v].data);
    }
    free(occ);
    free(p);
    free(ans);
    free(avail);

    return 0;
}
