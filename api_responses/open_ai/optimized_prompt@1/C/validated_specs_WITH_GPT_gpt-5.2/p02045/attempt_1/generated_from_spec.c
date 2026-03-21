#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int x;
    int y;
} Point;

static int iabs_int(int v) {
    /* Preserve typical C int behavior; abs(INT_MIN) is undefined in C,
       but the specification does not define overflow behavior. */
    return (v < 0) ? -v : v;
}

static int manhattan(Point a, Point b) {
    return iabs_int(a.x - b.x) + iabs_int(a.y - b.y);
}

/* Distance from point p to axis-aligned rectangle defined by [xmin,xmax] x [ymin,ymax].
   If bounds are "inverted", behavior follows the given bounds literally by using
   "distance outside the interval" with those bounds. */
static int dist_to_rect(Point p, int xmin, int xmax, int ymin, int ymax) {
    int dx = 0, dy = 0;

    if (p.x < xmin) dx = xmin - p.x;
    else if (p.x > xmax) dx = p.x - xmax;

    if (p.y < ymin) dy = ymin - p.y;
    else if (p.y > ymax) dy = p.y - ymax;

    return dx + dy;
}

int main(void) {
    int nseg;
    Point pts[128]; /* supports up to 127 segments => 128 points */

    for (;;) {
        int r = scanf("%d", &nseg);
        if (r != 1) {
            /* If initial segment count cannot be read: normal termination. */
            return 0;
        }
        if (nseg <= 0) {
            /* Non-positive segment count: stop processing normally. */
            return 0;
        }
        if (nseg > 127) {
            /* Specification says it won't exceed 127; if it does, narrowest
               safe behavior is to treat as input error and terminate. */
            return 1;
        }

        int xmin, xmax, ymin, ymax;
        if (scanf("%d %d %d %d", &xmin, &xmax, &ymin, &ymax) != 4) {
            /* Missing rectangle bounds: immediate error termination. */
            return 1;
        }

        for (int i = 0; i < nseg + 1; i++) {
            if (scanf("%d %d", &pts[i].x, &pts[i].y) != 2) {
                /* Missing point: immediate error termination. */
                return 1;
            }
        }

        long long total = 0;
        for (int i = 0; i < nseg; i++) {
            Point a = pts[i];
            Point b = pts[i + 1];

            int direct = manhattan(a, b);

            int da = dist_to_rect(a, xmin, xmax, ymin, ymax);
            int db = dist_to_rect(b, xmin, xmax, ymin, ymax);

            /* Escape discount: applies only to first point and only if outside (da > 0).
               Reduces by exactly 1 but never below zero. */
            if (da > 0) da -= 1;

            int via_rect = da + db;

            int seg_cost = (direct < via_rect) ? direct : via_rect;
            /* Per-segment costs are non-negative by construction. */
            total += (long long)seg_cost;
        }

        /* Output is specified as a single integer. Use long long to accumulate,
           but print as signed decimal; if it exceeds int range, behavior is
           still defined by the C library's formatting of long long. */
        printf("%lld\n", total);
    }
}
