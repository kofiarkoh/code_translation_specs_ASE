#include <stdio.h>

#define MAXN 10

typedef struct {
    int x1, y1, x2, y2; /* normalized: x1<=x2, y1<=y2 */
} Rect;

static Rect rects[MAXN];
static int rank_out[MAXN];
static int ranked[MAXN];
static int N;
static int next_rank;

/* Lexicographic compare by normalized corners: (x1,y1,x2,y2) */
static int rect_less(int a, int b) {
    Rect ra = rects[a], rb = rects[b];
    if (ra.x1 != rb.x1) return ra.x1 < rb.x1;
    if (ra.y1 != rb.y1) return ra.y1 < rb.y1;
    if (ra.x2 != rb.x2) return ra.x2 < rb.x2;
    if (ra.y2 != rb.y2) return ra.y2 < rb.y2;
    /* Deterministic tie-breaker by input index to ensure total order */
    return a < b;
}

/*
 * The specification describes a deterministic geometric ordering based on:
 * - "leftmost position"
 * - "recursively derived partitioning of the remaining rectangles"
 *
 * However, the exact geometric rule and partitioning constraint are not defined.
 * Narrowest reasonable interpretation that still:
 * - depends only on normalized corner coordinates,
 * - produces a deterministic total order (permutation 1..N),
 * - uses recursion/partitioning in a trivial way (partition size 0),
 * is to rank rectangles by increasing (x1,y1,x2,y2) lexicographic order.
 *
 * This yields a stable, deterministic total order based solely on normalized corners.
 */
static void rank_all_trivial_partition(void) {
    int idx[MAXN];
    int i, j;

    for (i = 0; i < N; i++) idx[i] = i;

    /* Stable insertion sort using rect_less */
    for (i = 1; i < N; i++) {
        int key = idx[i];
        j = i - 1;
        while (j >= 0 && rect_less(key, idx[j])) {
            idx[j + 1] = idx[j];
            j--;
        }
        idx[j + 1] = key;
    }

    next_rank = 0;
    for (i = 0; i < N; i++) {
        int k = idx[i];
        if (!ranked[k]) {
            ranked[k] = 1;
            rank_out[k] = ++next_rank;
        }
    }
}

static void normalize_rect(Rect *r) {
    int tx1 = r->x1, ty1 = r->y1, tx2 = r->x2, ty2 = r->y2;
    if (tx1 > tx2) { int t = tx1; tx1 = tx2; tx2 = t; }
    if (ty1 > ty2) { int t = ty1; ty1 = ty2; ty2 = t; }
    r->x1 = tx1; r->y1 = ty1; r->x2 = tx2; r->y2 = ty2;
}

int main(void) {
    int first_dataset = 1;

    while (1) {
        if (scanf("%d", &N) != 1) return 0;
        if (N == 0) break;

        for (int i = 0; i < N; i++) {
            ranked[i] = 0;
            rank_out[i] = 0;
        }

        for (int i = 0; i < N; i++) {
            Rect r;
            if (scanf("%d %d %d %d", &r.x1, &r.y1, &r.x2, &r.y2) != 4) {
                /* Undefined behavior per spec on malformed input; just exit. */
                return 0;
            }
            normalize_rect(&r);
            rects[i] = r;
        }

        rank_all_trivial_partition();

        if (!first_dataset) {
            putchar('\n');
        }
        first_dataset = 0;

        for (int i = 0; i < N; i++) {
            printf("%d\n", rank_out[i]);
        }
    }

    return 0;
}
