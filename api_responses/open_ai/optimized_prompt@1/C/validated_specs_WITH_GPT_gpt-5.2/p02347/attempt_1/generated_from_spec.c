#include <stdio.h>
#include <stdlib.h>

#ifndef MAX_POINTS
/* Fixed maximum capacity; specification does not provide a value.
   Choose a narrow, reasonable default typical for such tasks. */
#define MAX_POINTS 100000
#endif

typedef struct {
    int x, y;
    int id;
} Point;

static Point g_points[MAX_POINTS];
static int g_npoints = 0;

static int g_result_ids[MAX_POINTS];
static int g_result_count = 0;

static int cmp_int_asc(const void *a, const void *b) {
    /* Avoid overflow from subtraction; keep total order for int. */
    int ia = *(const int *)a;
    int ib = *(const int *)b;
    if (ia < ib) return -1;
    if (ia > ib) return 1;
    return 0;
}

/* Recursively scan points in [l, r) and collect matching ids.
   This intentionally uses recursion to match the specification's note. */
static void collect_ids_rec(int l, int r, int sx, int tx, int sy, int ty) {
    if (l >= r) return;
    if (r - l == 1) {
        Point p = g_points[l];
        if (p.x >= sx && p.x <= tx && p.y >= sy && p.y <= ty) {
            /* Per spec, behavior is undefined if capacity exceeded; we do not guard. */
            g_result_ids[g_result_count++] = p.id;
        }
        return;
    }
    int m = l + (r - l) / 2;
    collect_ids_rec(l, m, sx, tx, sy, ty);
    collect_ids_rec(m, r, sx, tx, sy, ty);
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    /* If n exceeds capacity, behavior is undefined per specification; proceed anyway. */
    g_npoints = n;

    for (int i = 0; i < n; i++) {
        int x, y;
        if (scanf("%d %d", &x, &y) != 2) return 0;
        if (i < MAX_POINTS) {
            g_points[i].x = x;
            g_points[i].y = y;
            g_points[i].id = i; /* 0-based identifier equals input position */
        } else {
            /* Out of capacity: undefined behavior allowed; ignore storage. */
        }
    }

    int q;
    if (scanf("%d", &q) != 1) return 0;

    for (int qi = 0; qi < q; qi++) {
        int sx, tx, sy, ty;
        if (scanf("%d %d %d %d", &sx, &tx, &sy, &ty) != 4) return 0;

        /* Per-query buffer cleared */
        g_result_count = 0;

        /* Only points stored within capacity are considered; exceeding capacity is UB anyway. */
        int effective_n = g_npoints;
        if (effective_n > MAX_POINTS) effective_n = MAX_POINTS;

        collect_ids_rec(0, effective_n, sx, tx, sy, ty);

        /* Sort identifiers ascending; duplicates cannot occur because each point is visited once. */
        qsort(g_result_ids, (size_t)g_result_count, sizeof(g_result_ids[0]), cmp_int_asc);

        for (int i = 0; i < g_result_count; i++) {
            printf("%d\n", g_result_ids[i]);
        }
        /* Blank line after each query, even if none listed */
        printf("\n");
    }

    return 0;
}
