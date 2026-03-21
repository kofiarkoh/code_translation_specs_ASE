#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *a;
    int head, tail; /* [head, tail) */
    int cap;
} Deque;

static void dq_init(Deque *d, int cap) {
    d->a = (int*)malloc((size_t)cap * sizeof(int));
    d->head = d->tail = 0;
    d->cap = cap;
}
static void dq_free(Deque *d) {
    free(d->a);
    d->a = NULL;
    d->head = d->tail = d->cap = 0;
}
static int dq_size(const Deque *d) { return d->tail - d->head; }
static int dq_empty(const Deque *d) { return d->tail == d->head; }
static int dq_front(const Deque *d) { return d->a[d->head]; }
static int dq_back(const Deque *d) { return d->a[d->tail - 1]; }
static void dq_push_back(Deque *d, int x) { d->a[d->tail++] = x; }
static int dq_pop_front(Deque *d) { return d->a[d->head++]; }
static int dq_pop_back(Deque *d) { return d->a[--d->tail]; }

/* SH monotone structures store values only (not indices), and are updated by equality on pop-front. */
static void sh_append(Deque *sh, Deque *shMin, Deque *shMax, int x) {
    dq_push_back(sh, x);

    while (!dq_empty(shMin) && dq_back(shMin) > x) (void)dq_pop_back(shMin);
    dq_push_back(shMin, x);

    while (!dq_empty(shMax) && dq_back(shMax) < x) (void)dq_pop_back(shMax);
    dq_push_back(shMax, x);
}

static int sh_pop_front(Deque *sh, Deque *shMin, Deque *shMax) {
    int x = dq_pop_front(sh);
    if (!dq_empty(shMin) && dq_front(shMin) == x) (void)dq_pop_front(shMin);
    if (!dq_empty(shMax) && dq_front(shMax) == x) (void)dq_pop_front(shMax);
    return x;
}

/* FH is a stack; prefix-extrema histories are stacks too. */
typedef struct {
    int *stk;
    int top; /* size */
    int *minHist;
    int minTop;
    int *maxHist;
    int maxTop;
    int cap;
} FH;

static void fh_init(FH *fh, int cap) {
    fh->stk = (int*)malloc((size_t)cap * sizeof(int));
    fh->minHist = (int*)malloc((size_t)cap * sizeof(int));
    fh->maxHist = (int*)malloc((size_t)cap * sizeof(int));
    fh->top = fh->minTop = fh->maxTop = 0;
    fh->cap = cap;
}
static void fh_free(FH *fh) {
    free(fh->stk);
    free(fh->minHist);
    free(fh->maxHist);
    fh->stk = fh->minHist = fh->maxHist = NULL;
    fh->top = fh->minTop = fh->maxTop = fh->cap = 0;
}
static int fh_size(const FH *fh) { return fh->top; }

static void fh_push(FH *fh, int x) {
    fh->stk[fh->top++] = x;

    if (fh->minTop == 0 || x < fh->minHist[fh->minTop - 1]) {
        fh->minHist[fh->minTop++] = x;
    }
    if (fh->maxTop == 0 || x > fh->maxHist[fh->maxTop - 1]) {
        fh->maxHist[fh->maxTop++] = x;
    }
}

static int fh_pop(FH *fh) {
    int x = fh->stk[--fh->top];
    if (fh->minTop > 0 && fh->minHist[fh->minTop - 1] == x) fh->minTop--;
    if (fh->maxTop > 0 && fh->maxHist[fh->maxTop - 1] == x) fh->maxTop--;
    return x;
}

/* k-th most recent in history: k=1 => last element. */
static int fh_query_min(const FH *fh, int k) { return fh->minHist[fh->minTop - k]; }
static int fh_query_max(const FH *fh, int k) { return fh->maxHist[fh->maxTop - k]; }

/* SH query: k-th position (1-based) in monotone deque from front. */
static int sh_query_min(const Deque *shMin, int k) { return shMin->a[shMin->head + (k - 1)]; }
static int sh_query_max(const Deque *shMax, int k) { return shMax->a[shMax->head + (k - 1)]; }

static void rebalance_if_needed(FH *fh, Deque *sh, Deque *shMin, Deque *shMax) {
    if (fh_size(fh) < dq_size(sh)) {
        int x = sh_pop_front(sh, shMin, shMax);
        fh_push(fh, x);
    }
}

int main(void) {
    int q;
    while (scanf("%d", &q) == 1) {
        if (q == 0) return 0;

        /* Capacity: specification says fixed internal capacity; we size to q+5 (narrowest reasonable). */
        int cap = q + 5;

        FH fh;
        fh_init(&fh, cap);

        Deque sh, shMin, shMax;
        dq_init(&sh, cap);
        dq_init(&shMin, cap);
        dq_init(&shMax, cap);

        for (int i = 0; i < q; i++) {
            int op;
            (void)scanf("%d", &op);

            if (op == 0) {
                int x;
                (void)scanf("%d", &x);
                sh_append(&sh, &shMin, &shMax, x);
                rebalance_if_needed(&fh, &sh, &shMin, &shMax);
            } else if (op == 1) {
                int x = fh_pop(&fh);
                rebalance_if_needed(&fh, &sh, &shMin, &shMax);
                printf("%d\n", x);
            } else if (op == 2) {
                printf("%d\n", fh_query_min(&fh, 1));
            } else if (op == 3) {
                printf("%d\n", sh_query_min(&shMin, 1));
            } else if (op == 4) {
                int k;
                (void)scanf("%d", &k);
                printf("%d\n", fh_query_min(&fh, k));
            } else if (op == 5) {
                int k;
                (void)scanf("%d", &k);
                printf("%d\n", sh_query_min(&shMin, k));
            } else if (op == 6) {
                printf("%d\n", fh_query_max(&fh, 1));
            } else if (op == 7) {
                printf("%d\n", sh_query_max(&shMax, 1));
            } else if (op == 8) {
                int k;
                (void)scanf("%d", &k);
                printf("%d\n", fh_query_max(&fh, k));
            } else if (op == 9) {
                int k;
                (void)scanf("%d", &k);
                printf("%d\n", sh_query_max(&shMax, k));
            } else {
                /* Undefined behavior per spec for malformed input/opcodes; do nothing. */
            }
        }

        printf("end\n");

        fh_free(&fh);
        dq_free(&sh);
        dq_free(&shMin);
        dq_free(&shMax);
    }
    return 0;
}
