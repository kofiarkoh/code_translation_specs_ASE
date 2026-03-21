#include <stdio.h>
#include <stdint.h>

#define MAXV 101

typedef struct {
    int64_t x, y;
} Pt;

static int64_t cross2(Pt a, Pt b) {
    return a.x * b.y - a.y * b.x;
}

static int64_t signed_area2_polygon(const Pt *p, int n) {
    int64_t s = 0;
    for (int i = 0; i < n; i++) {
        Pt a = p[i];
        Pt b = p[(i + 1) % n];
        s += cross2(a, b);
    }
    return s;
}

/* Sutherland–Hodgman clipping of a simple polygon against an axis-aligned rectangle.
   Returns the clipped polygon in out[], and its vertex count. */
static int clip_rect(const Pt *in, int n, int64_t xmin, int64_t xmax, int64_t ymin, int64_t ymax, Pt *out) {
    Pt buf1[4 * MAXV + 16];
    Pt buf2[4 * MAXV + 16];
    int m = n;

    for (int i = 0; i < n; i++) buf1[i] = in[i];

    /* Helper: intersect segment S->E with a vertical/horizontal line.
       Assumes the segment crosses the boundary (or touches), so denominator is non-zero
       unless the segment is parallel; in parallel cases, the caller should not request
       an intersection unless the segment lies on the boundary (in which case any point
       on it is acceptable; we choose E). */
    auto Pt intersect_vertical(Pt S, Pt E, int64_t xk) {
        int64_t dx = E.x - S.x;
        int64_t dy = E.y - S.y;
        if (dx == 0) return E; /* parallel; narrowest reasonable behavior */
        /* Use long double to avoid overflow in intermediate products; final is rounded to nearest integer
           by truncation toward zero when cast, matching typical integer-geometry expectations. */
        long double t = (long double)(xk - S.x) / (long double)dx;
        Pt I;
        I.x = xk;
        I.y = (int64_t)((long double)S.y + t * (long double)dy);
        return I;
    }

    auto Pt intersect_horizontal(Pt S, Pt E, int64_t yk) {
        int64_t dx = E.x - S.x;
        int64_t dy = E.y - S.y;
        if (dy == 0) return E; /* parallel; narrowest reasonable behavior */
        long double t = (long double)(yk - S.y) / (long double)dy;
        Pt I;
        I.y = yk;
        I.x = (int64_t)((long double)S.x + t * (long double)dx);
        return I;
    }

    /* Clip against x >= xmin (left) */
    {
        int k = 0;
        for (int i = 0; i < m; i++) {
            Pt S = buf1[i];
            Pt E = buf1[(i + 1) % m];
            int Sin = (S.x >= xmin);
            int Ein = (E.x >= xmin);
            if (Sin && Ein) {
                buf2[k++] = E;
            } else if (Sin && !Ein) {
                buf2[k++] = intersect_vertical(S, E, xmin);
            } else if (!Sin && Ein) {
                buf2[k++] = intersect_vertical(S, E, xmin);
                buf2[k++] = E;
            }
        }
        m = k;
        for (int i = 0; i < m; i++) buf1[i] = buf2[i];
    }

    /* Clip against x <= xmax (right) */
    if (m > 0) {
        int k = 0;
        for (int i = 0; i < m; i++) {
            Pt S = buf1[i];
            Pt E = buf1[(i + 1) % m];
            int Sin = (S.x <= xmax);
            int Ein = (E.x <= xmax);
            if (Sin && Ein) {
                buf2[k++] = E;
            } else if (Sin && !Ein) {
                buf2[k++] = intersect_vertical(S, E, xmax);
            } else if (!Sin && Ein) {
                buf2[k++] = intersect_vertical(S, E, xmax);
                buf2[k++] = E;
            }
        }
        m = k;
        for (int i = 0; i < m; i++) buf1[i] = buf2[i];
    }

    /* Clip against y >= ymin (bottom) */
    if (m > 0) {
        int k = 0;
        for (int i = 0; i < m; i++) {
            Pt S = buf1[i];
            Pt E = buf1[(i + 1) % m];
            int Sin = (S.y >= ymin);
            int Ein = (E.y >= ymin);
            if (Sin && Ein) {
                buf2[k++] = E;
            } else if (Sin && !Ein) {
                buf2[k++] = intersect_horizontal(S, E, ymin);
            } else if (!Sin && Ein) {
                buf2[k++] = intersect_horizontal(S, E, ymin);
                buf2[k++] = E;
            }
        }
        m = k;
        for (int i = 0; i < m; i++) buf1[i] = buf2[i];
    }

    /* Clip against y <= ymax (top) */
    if (m > 0) {
        int k = 0;
        for (int i = 0; i < m; i++) {
            Pt S = buf1[i];
            Pt E = buf1[(i + 1) % m];
            int Sin = (S.y <= ymax);
            int Ein = (E.y <= ymax);
            if (Sin && Ein) {
                buf2[k++] = E;
            } else if (Sin && !Ein) {
                buf2[k++] = intersect_horizontal(S, E, ymax);
            } else if (!Sin && Ein) {
                buf2[k++] = intersect_horizontal(S, E, ymax);
                buf2[k++] = E;
            }
        }
        m = k;
        for (int i = 0; i < m; i++) buf1[i] = buf2[i];
    }

    for (int i = 0; i < m; i++) out[i] = buf1[i];
    return m;
}

int main(void) {
    int n;
    while (scanf("%d", &n) == 1) {
        if (n == 0) break;

        Pt poly[MAXV];
        for (int i = 0; i < n; i++) {
            long long x, y;
            scanf("%lld %lld", &x, &y);
            poly[i].x = (int64_t)x;
            poly[i].y = (int64_t)y;
        }

        int64_t xmin = 0, xmax = 0, ymin = 0, ymax = 0;
        for (int i = 0; i < 4; i++) {
            long long x, y;
            scanf("%lld %lld", &x, &y);
            int64_t xi = (int64_t)x, yi = (int64_t)y;
            if (i == 0) {
                xmin = xmax = xi;
                ymin = ymax = yi;
            } else {
                if (xi < xmin) xmin = xi;
                if (xi > xmax) xmax = xi;
                if (yi < ymin) ymin = yi;
                if (yi > ymax) ymax = yi;
            }
        }

        int64_t area2_total = signed_area2_polygon(poly, n);

        Pt clipped[4 * MAXV + 16];
        int m = clip_rect(poly, n, xmin, xmax, ymin, ymax, clipped);
        int64_t area2_in = (m >= 3) ? signed_area2_polygon(clipped, m) : 0;

        int64_t area2_out = area2_total - area2_in;
        printf("%lld\n", (long long)area2_out);
    }
    return 0;
}
