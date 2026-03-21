#include <stdio.h>
#include <stdint.h>

#define MAXN 100000
#define MAXQ 100000

/* Segment tree with range assignment (set) and range sum query.
   Uses a sentinel to represent "no pending assignment" on a node.
   Per spec, input x will never cause confusion with this sentinel. */
static int64_t seg[4 * MAXN + 5];
static int64_t lazy[4 * MAXN + 5];
static const int64_t LAZY_NONE = (int64_t)0x7fffffffffffffffLL; /* sentinel */

static void build_init(int idx, int l, int r) {
    seg[idx] = 0;
    lazy[idx] = LAZY_NONE;
    if (l == r) return;
    int mid = l + (r - l) / 2;
    build_init(idx * 2, l, mid);
    build_init(idx * 2 + 1, mid + 1, r);
}

static void push(int idx, int l, int r) {
    if (lazy[idx] == LAZY_NONE) return;
    int64_t v = lazy[idx];
    seg[idx] = v * (int64_t)(r - l + 1);
    if (l != r) {
        lazy[idx * 2] = v;
        lazy[idx * 2 + 1] = v;
    }
    lazy[idx] = LAZY_NONE;
}

static void range_assign(int idx, int l, int r, int ql, int qr, int64_t v) {
    push(idx, l, r);
    if (qr < l || r < ql) return;
    if (ql <= l && r <= qr) {
        lazy[idx] = v;
        push(idx, l, r);
        return;
    }
    int mid = l + (r - l) / 2;
    range_assign(idx * 2, l, mid, ql, qr, v);
    range_assign(idx * 2 + 1, mid + 1, r, ql, qr, v);
    seg[idx] = seg[idx * 2] + seg[idx * 2 + 1];
}

static int64_t range_sum(int idx, int l, int r, int ql, int qr) {
    push(idx, l, r);
    if (qr < l || r < ql) return 0;
    if (ql <= l && r <= qr) return seg[idx];
    int mid = l + (r - l) / 2;
    return range_sum(idx * 2, l, mid, ql, qr) +
           range_sum(idx * 2 + 1, mid + 1, r, ql, qr);
}

int main(void) {
    int num, queries;
    if (scanf("%d %d", &num, &queries) != 2) return 0;

    if (num > 0) build_init(1, 0, num - 1);

    for (int qi = 0; qi < queries; qi++) {
        int type;
        int start, end;
        if (scanf("%d %d %d", &type, &start, &end) != 3) return 0;

        /* Unrecognized operation types are ignored (no output, no state change). */
        if (type == 0) {
            /* Interpret type==0 as update: requires extra integer x. */
            int64_t x;
            if (scanf("%" SCNd64, &x) != 1) return 0;
            if (num > 0) range_assign(1, 0, num - 1, start, end, x);
        } else if (type == 1) {
            /* Interpret type==1 as sum query. */
            int64_t ans = 0;
            if (num > 0) ans = range_sum(1, 0, num - 1, start, end);
            printf("%" PRId64 "\n", ans);
        } else {
            /* Ignore unknown type; do not attempt to read an extra x. */
        }
    }

    return 0;
}
