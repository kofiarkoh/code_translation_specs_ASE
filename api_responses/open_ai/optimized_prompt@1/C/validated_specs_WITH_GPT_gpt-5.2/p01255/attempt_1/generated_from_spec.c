#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

/*
  Translation notes (kept as C comments per instructions):

  - Fixed-size storage is implied by the specification; exact bounds are not given.
    We choose conservative "small" bounds. If input exceeds them, behavior is undefined,
    matching the spec.

  - Conflict definition: two routes conflict if they both pass through at least one
    common geometric point, including shared endpoints and proper intersections.

  - The spec describes building a set of all segment endpoints and all "computable"
    pairwise segment intersection points, with a fixed tolerance for point equality.
    We implement:
      * endpoints are always added as points
      * for each pair of segments from different routes, compute a single intersection
        point if they properly intersect (including touching at endpoints) and are not
        parallel/collinear; for parallel/collinear we do not attempt to generate points
        beyond endpoints (as per spec's non-robust handling).

  - Then, for each derived point, we determine which routes "pass through" it:
      * if the point matches any vertex of the route within tolerance, or
      * if the point lies on any segment of the route within tolerance.
    Any pair of routes co-occurring at a point are marked conflicting.

  - A route is considered conflicting with itself for heuristic bookkeeping:
    we set adjacency[i][i] = 1.

  - Greedy coloring heuristic: Welsh–Powell variant.
    We interpret this as:
      1) order vertices by non-increasing degree (ties by index)
      2) then color greedily in that order with smallest available color.
    The spec requires two traversal orders (forward and reverse) over the same
    conflict graph; we implement:
      * compute the sorted order once
      * run greedy in that order (forward)
      * run greedy in reverse of that order
    Output the smaller number of colors used.

  - Floating tolerance: fixed small tolerance EPS.
*/

#define MAX_ROUTES   64
#define MAX_PTS_PER_ROUTE 256
#define MAX_SEGS     (MAX_ROUTES * (MAX_PTS_PER_ROUTE > 0 ? (MAX_PTS_PER_ROUTE - 1) : 0))
/* Derived points: endpoints + pairwise intersections (worst-case quadratic in segments).
   Keep a fixed cap; exceeding is undefined per spec. */
#define MAX_DERIVED_POINTS 200000

static const double EPS = 1e-7;

typedef struct { double x, y; } DPoint;

typedef struct {
    int route_id;
    DPoint a, b;
} Segment;

typedef struct {
    int npts;
    DPoint pts[MAX_PTS_PER_ROUTE];
} Route;

static int dpoint_eq(DPoint p, DPoint q) {
    return (fabs(p.x - q.x) <= EPS) && (fabs(p.y - q.y) <= EPS);
}

static double cross2(double ax, double ay, double bx, double by) {
    return ax * by - ay * bx;
}

static double dot2(double ax, double ay, double bx, double by) {
    return ax * bx + ay * by;
}

static int point_on_segment(DPoint p, DPoint a, DPoint b) {
    /* Check collinearity and within bounding box with tolerance. */
    double abx = b.x - a.x, aby = b.y - a.y;
    double apx = p.x - a.x, apy = p.y - a.y;
    double cr = cross2(abx, aby, apx, apy);
    if (fabs(cr) > 1e-6) return 0; /* slightly looser than EPS for robustness */
    double dp = dot2(apx, apy, abx, aby);
    if (dp < -1e-6) return 0;
    double ab2 = dot2(abx, aby, abx, aby);
    if (dp - ab2 > 1e-6) return 0;
    return 1;
}

static int segment_intersection_point(DPoint a, DPoint b, DPoint c, DPoint d, DPoint *out) {
    /* Compute a single intersection point for non-parallel lines, then check if it lies on both segments.
       For parallel/collinear, return 0 (non-robust handling per spec). */
    double r_x = b.x - a.x, r_y = b.y - a.y;
    double s_x = d.x - c.x, s_y = d.y - c.y;
    double denom = cross2(r_x, r_y, s_x, s_y);

    if (fabs(denom) <= 1e-12) {
        /* Parallel or collinear: do not compute intersection point beyond endpoints. */
        return 0;
    }

    double cma_x = c.x - a.x, cma_y = c.y - a.y;
    double t = cross2(cma_x, cma_y, s_x, s_y) / denom;
    double u = cross2(cma_x, cma_y, r_x, r_y) / denom;

    /* Allow small tolerance for endpoint touching. */
    if (t < -1e-9 || t > 1.0 + 1e-9 || u < -1e-9 || u > 1.0 + 1e-9) return 0;

    out->x = a.x + t * r_x;
    out->y = a.y + t * r_y;
    return 1;
}

static int add_unique_point(DPoint *pts, int *n, DPoint p) {
    for (int i = 0; i < *n; i++) {
        if (dpoint_eq(pts[i], p)) return i;
    }
    if (*n >= MAX_DERIVED_POINTS) {
        /* Undefined behavior per spec; we just stop adding. */
        return -1;
    }
    pts[*n] = p;
    (*n)++;
    return (*n) - 1;
}

static int route_passes_point(const Route *r, DPoint p) {
    /* Vertex match */
    for (int i = 0; i < r->npts; i++) {
        if (dpoint_eq(r->pts[i], p)) return 1;
    }
    /* Segment containment */
    for (int i = 0; i + 1 < r->npts; i++) {
        if (point_on_segment(p, r->pts[i], r->pts[i + 1])) return 1;
    }
    return 0;
}

static int greedy_color_count(int n, const unsigned char adj[MAX_ROUTES][MAX_ROUTES], const int *order, int reverse) {
    int color[MAX_ROUTES];
    for (int i = 0; i < n; i++) color[i] = -1;

    int max_color = -1;

    for (int pos = 0; pos < n; pos++) {
        int v = reverse ? order[n - 1 - pos] : order[pos];

        /* Find smallest color not used by neighbors already colored. */
        int used[MAX_ROUTES];
        for (int i = 0; i < n; i++) used[i] = 0;

        for (int u = 0; u < n; u++) {
            if (adj[v][u] && color[u] >= 0) used[color[u]] = 1;
        }

        int c;
        for (c = 0; c < n; c++) {
            if (!used[c]) break;
        }
        color[v] = c;
        if (c > max_color) max_color = c;
    }

    return max_color + 1;
}

static int cmp_degree_desc_then_index(const void *pa, const void *pb, void *ctx) {
    const int *deg = (const int *)ctx;
    int a = *(const int *)pa;
    int b = *(const int *)pb;
    if (deg[a] != deg[b]) return (deg[b] - deg[a]);
    return (a - b);
}

int main(void) {
    int n_routes;
    while (scanf("%d", &n_routes) == 1) {
        if (n_routes == 0) break;

        Route routes[MAX_ROUTES];
        if (n_routes > MAX_ROUTES) {
            /* Undefined per spec; still attempt to read but will overflow if continued. */
        }

        for (int i = 0; i < n_routes; i++) {
            int m;
            scanf("%d", &m);
            routes[i].npts = m;
            for (int j = 0; j < m; j++) {
                long long xi, yi;
                scanf("%lld %lld", &xi, &yi);
                routes[i].pts[j].x = (double)xi;
                routes[i].pts[j].y = (double)yi;
            }
        }

        /* Build segment list */
        Segment segs[MAX_SEGS];
        int n_segs = 0;
        for (int i = 0; i < n_routes; i++) {
            for (int j = 0; j + 1 < routes[i].npts; j++) {
                if (n_segs >= MAX_SEGS) {
                    /* Undefined per spec; stop adding. */
                    break;
                }
                segs[n_segs].route_id = i;
                segs[n_segs].a = routes[i].pts[j];
                segs[n_segs].b = routes[i].pts[j + 1];
                n_segs++;
            }
        }

        /* Build derived points: all endpoints + all computable intersections */
        DPoint derived[MAX_DERIVED_POINTS];
        int n_derived = 0;

        for (int i = 0; i < n_routes; i++) {
            for (int j = 0; j < routes[i].npts; j++) {
                add_unique_point(derived, &n_derived, routes[i].pts[j]);
            }
        }

        for (int i = 0; i < n_segs; i++) {
            for (int j = i + 1; j < n_segs; j++) {
                if (segs[i].route_id == segs[j].route_id) continue;
                DPoint ip;
                if (segment_intersection_point(segs[i].a, segs[i].b, segs[j].a, segs[j].b, &ip)) {
                    add_unique_point(derived, &n_derived, ip);
                }
            }
        }

        /* Build conflict graph */
        unsigned char adj[MAX_ROUTES][MAX_ROUTES];
        memset(adj, 0, sizeof(adj));
        for (int i = 0; i < n_routes; i++) adj[i][i] = 1; /* self-conflict invariant */

        for (int pi = 0; pi < n_derived; pi++) {
            int present[MAX_ROUTES];
            for (int r = 0; r < n_routes; r++) present[r] = 0;

            for (int r = 0; r < n_routes; r++) {
                if (routes[r].npts == 0) continue;
                if (route_passes_point(&routes[r], derived[pi])) present[r] = 1;
            }

            for (int a = 0; a < n_routes; a++) if (present[a]) {
                for (int b = a + 1; b < n_routes; b++) if (present[b]) {
                    adj[a][b] = adj[b][a] = 1;
                }
            }
        }

        /* Degree and Welsh–Powell ordering */
        int deg[MAX_ROUTES];
        for (int i = 0; i < n_routes; i++) {
            int d = 0;
            for (int j = 0; j < n_routes; j++) if (adj[i][j] && i != j) d++;
            deg[i] = d;
        }

        int order[MAX_ROUTES];
        for (int i = 0; i < n_routes; i++) order[i] = i;

        /* qsort_r is non-standard; implement stable-ish sort with simple O(n^2) due to small bounds. */
        for (int i = 0; i < n_routes; i++) {
            for (int j = i + 1; j < n_routes; j++) {
                int a = order[i], b = order[j];
                if (deg[a] < deg[b] || (deg[a] == deg[b] && a > b)) {
                    int tmp = order[i]; order[i] = order[j]; order[j] = tmp;
                }
            }
        }

        int c1 = greedy_color_count(n_routes, adj, order, 0);
        int c2 = greedy_color_count(n_routes, adj, order, 1);
        int ans = (c1 < c2) ? c1 : c2;

        printf("%d\n", ans);
    }
    return 0;
}
