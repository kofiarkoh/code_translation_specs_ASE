#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

#define CAP 400000  /* fixed internal capacity per structure; spec says ~200k/half, undefined if exceeded */

static int read_uint(int *out) {
    int c;
    do {
        c = getchar();
        if (c == EOF) return 0;
    } while (!isdigit((unsigned char)c));
    unsigned int v = 0;
    do {
        v = v * 10u + (unsigned int)(c - '0');
        c = getchar();
    } while (c != EOF && isdigit((unsigned char)c));
    *out = (int)v; /* spec: all values fit in int */
    return 1;
}

/* Simple deque for ints using circular buffer */
typedef struct {
    int a[CAP];
    int head, tail; /* [head, tail) */
} Deque;

static void dq_clear(Deque *d) { d->head = d->tail = 0; }
static int dq_size(const Deque *d) { return (d->tail - d->head + CAP) % CAP; }
static int dq_empty(const Deque *d) { return d->head == d->tail; }
static int dq_front(const Deque *d) { return d->a[d->head]; }
static int dq_back(const Deque *d) { return d->a[(d->tail - 1 + CAP) % CAP]; }
static void dq_push_back(Deque *d, int x) { d->a[d->tail] = x; d->tail = (d->tail + 1) % CAP; }
static int dq_pop_front(Deque *d) { int x = d->a[d->head]; d->head = (d->head + 1) % CAP; return x; }
static int dq_pop_back(Deque *d) { d->tail = (d->tail - 1 + CAP) % CAP; return d->a[d->tail]; }
static int dq_get_k_from_front(const Deque *d, int k1) {
    /* k1 is 1-based */
    int idx = (d->head + (k1 - 1)) % CAP;
    return d->a[idx];
}

/* Stack-like array for FH record sequences */
typedef struct {
    int a[CAP];
    int n;
} Stack;

static void st_clear(Stack *s) { s->n = 0; }
static void st_push(Stack *s, int x) { s->a[s->n++] = x; }
static void st_pop(Stack *s) { s->n--; }
static int st_top(const Stack *s) { return s->a[s->n - 1]; }
static int st_get_k_from_end(const Stack *s, int k1) {
    /* k1 is 1-based from end */
    return s->a[s->n - k1];
}

/* FH actual elements as a stack (prefix of sequence) */
typedef struct {
    int a[CAP];
    int n;
} ArrStack;

static void as_clear(ArrStack *s) { s->n = 0; }
static void as_push(ArrStack *s, int x) { s->a[s->n++] = x; }
static int as_pop(ArrStack *s) { return s->a[--s->n]; }
static int as_top(const ArrStack *s) { return s->a[s->n - 1]; }

/*
   FH record maintenance:
   - prefix minima records: strictly decreasing values when scanning left->right
     => on append to FH, if x < last_record_min then push x
   - prefix maxima records: strictly increasing values when scanning left->right
     => on append to FH, if x > last_record_max then push x
   On pop from back of FH, if popped value equals last record value, pop record.
*/
static void fh_append(int x, ArrStack *fh, Stack *fh_minrec, Stack *fh_maxrec) {
    as_push(fh, x);
    if (fh_minrec->n == 0 || x < st_top(fh_minrec)) st_push(fh_minrec, x);
    if (fh_maxrec->n == 0 || x > st_top(fh_maxrec)) st_push(fh_maxrec, x);
}

static int fh_pop_back(ArrStack *fh, Stack *fh_minrec, Stack *fh_maxrec) {
    int x = as_pop(fh);
    if (fh_minrec->n > 0 && x == st_top(fh_minrec)) st_pop(fh_minrec);
    if (fh_maxrec->n > 0 && x == st_top(fh_maxrec)) st_pop(fh_maxrec);
    return x;
}

/*
   SH maintenance:
   - actual SH elements in a deque (for moving front to FH)
   - min deque: nondecreasing; on insert x at back, pop back while back > x, then push x
   - max deque: nonincreasing; on insert x at back, pop back while back < x, then push x
   - when removing from front of SH, if removed equals front of min/max deque, pop front there too
*/
static void sh_push_back(int x, Deque *sh, Deque *sh_min, Deque *sh_max) {
    dq_push_back(sh, x);

    while (!dq_empty(sh_min) && dq_back(sh_min) > x) dq_pop_back(sh_min);
    dq_push_back(sh_min, x);

    while (!dq_empty(sh_max) && dq_back(sh_max) < x) dq_pop_back(sh_max);
    dq_push_back(sh_max, x);
}

static int sh_pop_front(Deque *sh, Deque *sh_min, Deque *sh_max) {
    int x = dq_pop_front(sh);
    if (!dq_empty(sh_min) && dq_front(sh_min) == x) dq_pop_front(sh_min);
    if (!dq_empty(sh_max) && dq_front(sh_max) == x) dq_pop_front(sh_max);
    return x;
}

static void rebalance_if_needed(ArrStack *fh, Stack *fh_minrec, Stack *fh_maxrec,
                                Deque *sh, Deque *sh_min, Deque *sh_max) {
    /* Only rebalancing described by spec: if |FH| < |SH|, move one from front(SH) to back(FH) */
    if (fh->n < dq_size(sh)) {
        int x = sh_pop_front(sh, sh_min, sh_max);
        fh_append(x, fh, fh_minrec, fh_maxrec);
    }
}

int main(void) {
    int q;
    while (read_uint(&q)) {
        if (q == 0) return 0;

        ArrStack fh;
        Stack fh_minrec, fh_maxrec;
        Deque sh, sh_min, sh_max;

        as_clear(&fh);
        st_clear(&fh_minrec);
        st_clear(&fh_maxrec);
        dq_clear(&sh);
        dq_clear(&sh_min);
        dq_clear(&sh_max);

        for (int i = 0; i < q; i++) {
            int op;
            if (!read_uint(&op)) return 0; /* undefined per spec; just stop */

            if (op == 0) {
                int x;
                read_uint(&x);
                sh_push_back(x, &sh, &sh_min, &sh_max);
                rebalance_if_needed(&fh, &fh_minrec, &fh_maxrec, &sh, &sh_min, &sh_max);
            } else if (op == 1) {
                int x = fh_pop_back(&fh, &fh_minrec, &fh_maxrec);
                printf("%d\n", x);
                rebalance_if_needed(&fh, &fh_minrec, &fh_maxrec, &sh, &sh_min, &sh_max);
            } else if (op == 2) {
                printf("%d\n", st_get_k_from_end(&fh_minrec, 1));
            } else if (op == 3) {
                printf("%d\n", dq_get_k_from_front(&sh_min, 1));
            } else if (op == 4) {
                int k;
                read_uint(&k);
                printf("%d\n", st_get_k_from_end(&fh_minrec, k));
            } else if (op == 5) {
                int k;
                read_uint(&k);
                printf("%d\n", dq_get_k_from_front(&sh_min, k));
            } else if (op == 6) {
                printf("%d\n", st_get_k_from_end(&fh_maxrec, 1));
            } else if (op == 7) {
                printf("%d\n", dq_get_k_from_front(&sh_max, 1));
            } else if (op == 8) {
                int k;
                read_uint(&k);
                printf("%d\n", st_get_k_from_end(&fh_maxrec, k));
            } else if (op == 9) {
                int k;
                read_uint(&k);
                printf("%d\n", dq_get_k_from_front(&sh_max, k));
            } else {
                /* undefined per spec; ignore */
            }
        }

        puts("end");
    }
    return 0;
}
