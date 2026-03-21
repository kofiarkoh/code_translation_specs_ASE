#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *a;
    int n;
    int cap;
} IntVec;

static void vec_init(IntVec *v, int cap) {
    v->a = (int*)malloc((size_t)cap * sizeof(int));
    v->n = 0;
    v->cap = cap;
}

static void vec_free(IntVec *v) {
    free(v->a);
    v->a = NULL;
    v->n = 0;
    v->cap = 0;
}

static void vec_clear(IntVec *v) { v->n = 0; }

static void vec_push(IntVec *v, int x) {
    /* Capacity is assumed sufficient per spec; no checks. */
    v->a[v->n++] = x;
}

static int vec_pop(IntVec *v) {
    return v->a[--v->n];
}

static int vec_top(const IntVec *v) {
    return v->a[v->n - 1];
}

/*
  Data structure:
  - FH: stack (top at end of array)
  - SH: queue implemented as circular buffer
  Each half maintains "record minima" and "record maxima" sequences for the scan direction:
    FH scan: from top backward (i.e., from end to start)
    SH scan: from front forward
  We maintain these record sequences incrementally with push/pop operations at the relevant ends.
*/

typedef struct {
    IntVec data;     /* stack elements */
    IntVec recMin;   /* record minima sequence for scan from top backward */
    IntVec recMax;   /* record maxima sequence for scan from top backward */
} FHStack;

static void fh_init(FHStack *fh, int cap) {
    vec_init(&fh->data, cap);
    vec_init(&fh->recMin, cap);
    vec_init(&fh->recMax, cap);
}

static void fh_free(FHStack *fh) {
    vec_free(&fh->data);
    vec_free(&fh->recMin);
    vec_free(&fh->recMax);
}

static void fh_clear(FHStack *fh) {
    vec_clear(&fh->data);
    vec_clear(&fh->recMin);
    vec_clear(&fh->recMax);
}

static int fh_size(const FHStack *fh) { return fh->data.n; }

static void fh_push(FHStack *fh, int x) {
    vec_push(&fh->data, x);

    /* Update record minima for scan from top backward:
       New top is encountered first; it is always the first record.
       It becomes a new record minimum iff it is strictly smaller than previous first record. */
    if (fh->recMin.n == 0 || x < vec_top(&fh->recMin)) vec_push(&fh->recMin, x);

    /* Update record maxima similarly. */
    if (fh->recMax.n == 0 || x > vec_top(&fh->recMax)) vec_push(&fh->recMax, x);
}

static int fh_pop(FHStack *fh) {
    int x = vec_pop(&fh->data);

    /* If the popped element was the current first record (stored at end), remove it. */
    if (fh->recMin.n > 0 && x == vec_top(&fh->recMin)) (void)vec_pop(&fh->recMin);
    if (fh->recMax.n > 0 && x == vec_top(&fh->recMax)) (void)vec_pop(&fh->recMax);

    return x;
}

static int fh_query_recmin_k(const FHStack *fh, int k) {
    /* k is 1-based; recMin is stored in encounter order, with first record at index 0,
       but we appended records as they appear at the top over time, so the first record
       is at recMin[0] and the k-th is recMin[k-1]. */
    return fh->recMin.a[k - 1];
}

static int fh_query_recmax_k(const FHStack *fh, int k) {
    return fh->recMax.a[k - 1];
}

typedef struct {
    int *a;
    int cap;
    int head; /* index of front */
    int tail; /* index one past last */
    int n;
    IntVec recMin; /* record minima sequence for scan from front forward */
    IntVec recMax; /* record maxima sequence for scan from front forward */
} SHQueue;

static void sh_init(SHQueue *sh, int cap) {
    sh->a = (int*)malloc((size_t)cap * sizeof(int));
    sh->cap = cap;
    sh->head = 0;
    sh->tail = 0;
    sh->n = 0;
    vec_init(&sh->recMin, cap);
    vec_init(&sh->recMax, cap);
}

static void sh_free(SHQueue *sh) {
    free(sh->a);
    sh->a = NULL;
    sh->cap = 0;
    sh->head = sh->tail = sh->n = 0;
    vec_free(&sh->recMin);
    vec_free(&sh->recMax);
}

static void sh_clear(SHQueue *sh) {
    sh->head = sh->tail = sh->n = 0;
    vec_clear(&sh->recMin);
    vec_clear(&sh->recMax);
}

static int sh_size(const SHQueue *sh) { return sh->n; }

static void sh_push_back(SHQueue *sh, int x) {
    /* Append to back. */
    sh->a[sh->tail] = x;
    sh->tail++;
    if (sh->tail == sh->cap) sh->tail = 0;
    sh->n++;

    /* Update record sequences for scan from front forward:
       Appending at back affects record sequences only if it creates a new record
       after scanning all previous elements. That happens if it is strictly smaller
       than the last record minimum, or strictly larger than the last record maximum. */
    if (sh->recMin.n == 0 || x < vec_top(&sh->recMin)) vec_push(&sh->recMin, x);
    if (sh->recMax.n == 0 || x > vec_top(&sh->recMax)) vec_push(&sh->recMax, x);
}

static int sh_pop_front(SHQueue *sh) {
    int x = sh->a[sh->head];
    sh->head++;
    if (sh->head == sh->cap) sh->head = 0;
    sh->n--;

    /* Removing from front can invalidate record sequences; if the removed element
       equals the first record, we must recompute the entire record sequence from
       the remaining queue in scan order. This is O(n) but faithful. */
    if ((sh->recMin.n > 0 && x == sh->recMin.a[0]) ||
        (sh->recMax.n > 0 && x == sh->recMax.a[0])) {

        vec_clear(&sh->recMin);
        vec_clear(&sh->recMax);

        int idx = sh->head;
        for (int i = 0; i < sh->n; i++) {
            int v = sh->a[idx];
            idx++;
            if (idx == sh->cap) idx = 0;

            if (sh->recMin.n == 0 || v < vec_top(&sh->recMin)) vec_push(&sh->recMin, v);
            if (sh->recMax.n == 0 || v > vec_top(&sh->recMax)) vec_push(&sh->recMax, v);
        }
    }

    return x;
}

static int sh_query_recmin_k(const SHQueue *sh, int k) {
    return sh->recMin.a[k - 1];
}

static int sh_query_recmax_k(const SHQueue *sh, int k) {
    return sh->recMax.a[k - 1];
}

static void rebalance_if_needed(FHStack *fh, SHQueue *sh) {
    /* Rebalancing moves only from SH front to FH top, and only when FH < SH. */
    if (fh_size(fh) < sh_size(sh)) {
        int x = sh_pop_front(sh);
        fh_push(fh, x);
    }
}

int main(void) {
    int q;
    while (scanf("%d", &q) == 1) {
        if (q == 0) return 0;

        /* Fixed internal capacity per test case: at most q elements can be stored. */
        int cap = q + 5;

        FHStack fh;
        SHQueue sh;
        fh_init(&fh, cap);
        sh_init(&sh, cap);

        for (int i = 0; i < q; i++) {
            int op;
            scanf("%d", &op);

            if (op == 0) {
                int x;
                scanf("%d", &x);
                sh_push_back(&sh, x);
                rebalance_if_needed(&fh, &sh);
            } else if (op == 1) {
                int x = fh_pop(&fh);
                printf("%d\n", x);
                rebalance_if_needed(&fh, &sh);
            } else if (op == 2) {
                printf("%d\n", fh_query_recmin_k(&fh, 1));
            } else if (op == 3) {
                printf("%d\n", sh_query_recmin_k(&sh, 1));
            } else if (op == 4) {
                int k;
                scanf("%d", &k);
                printf("%d\n", fh_query_recmin_k(&fh, k));
            } else if (op == 5) {
                int k;
                scanf("%d", &k);
                printf("%d\n", sh_query_recmin_k(&sh, k));
            } else if (op == 6) {
                printf("%d\n", fh_query_recmax_k(&fh, 1));
            } else if (op == 7) {
                printf("%d\n", sh_query_recmax_k(&sh, 1));
            } else if (op == 8) {
                int k;
                scanf("%d", &k);
                printf("%d\n", fh_query_recmax_k(&fh, k));
            } else if (op == 9) {
                int k;
                scanf("%d", &k);
                printf("%d\n", sh_query_recmax_k(&sh, k));
            }
        }

        printf("end\n");

        fh_free(&fh);
        sh_free(&sh);
    }
    return 0;
}
