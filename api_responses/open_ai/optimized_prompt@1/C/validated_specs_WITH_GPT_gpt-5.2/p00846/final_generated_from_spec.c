#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef struct {
    double x, y;
} Pt;

static const double EPS = 1e-9;

static double cross2(double ax, double ay, double bx, double by) {
    return ax * by - ay * bx;
}

/* Signed area *2 of triangle (a,b,p): positive if p is to the left of directed edge a->b */
static double side(const Pt a, const Pt b, const Pt p) {
    return cross2(b.x - a.x, b.y - a.y, p.x - a.x, p.y - a.y);
}

/* Intersect segment s->e with the infinite line through a->b.
   Precondition from spec: when needed, intersection is uniquely defined (not parallel in the crossing case). */
static Pt intersect_line(const Pt s, const Pt e, const Pt a, const Pt b) {
    /* Solve for t where s + t*(e-s) lies on line a->b:
       cross(b-a, (s + t*(e-s)) - a) = 0
       => cross(b-a, s-a) + t*cross(b-a, e-s) = 0
       => t = -cross(b-a, s-a) / cross(b-a, e-s)
    */
    double bax = b.x - a.x, bay = b.y - a.y;
    double sax = s.x - a.x, say = s.y - a.y;
    double esx = e.x - s.x, esy = e.y - s.y;

    double num = cross2(bax, bay, sax, say);
    double den = cross2(bax, bay, esx, esy);

    /* If den is 0, lines are parallel; spec says this won't be required for a crossing intersection. */
    double t;
    if (fabs(den) < 1e-18) {
        /* Narrowest reasonable behavior: return s (arbitrary) to keep code defined;
           this path should not be taken under preconditions when intersection is needed. */
        t = 0.0;
    } else {
        t = -num / den;
    }

    Pt r;
    r.x = s.x + t * (e.x - s.x);
    r.y = s.y + t * (e.y - s.y);
    return r;
}

/* Clip convex/possibly-nonconvex polygon 'in' by half-plane left of directed edge a->b.
   Uses Sutherland–Hodgman polygon clipping. */
static int clip_halfplane(const Pt *in, int n_in, Pt *out, const Pt a, const Pt b) {
    if (n_in <= 0) return 0;

    int n_out = 0;
    Pt S = in[n_in - 1];
    double sS = side(a, b, S);

    for (int i = 0; i < n_in; i++) {
        Pt E = in[i];
        double sE = side(a, b, E);

        int Ein = (sE >= -EPS);
        int Sin = (sS >= -EPS);

        if (Ein) {
            if (!Sin) {
                /* entering: add intersection then E */
                Pt I = intersect_line(S, E, a, b);
                out[n_out++] = I;
            }
            out[n_out++] = E;
        } else {
            if (Sin) {
                /* leaving: add intersection only */
                Pt I = intersect_line(S, E, a, b);
                out[n_out++] = I;
            }
        }

        S = E;
        sS = sE;
    }

    return n_out;
}

int main(void) {
    int n;
    while (scanf("%d", &n) == 1) {
        if (n <= 0) break;

        Pt *poly = (Pt *)malloc((size_t)n * sizeof(Pt));
        if (!poly) return 0;

        for (int i = 0; i < n; i++) {
            long long xi, yi;
            if (scanf("%lld %lld", &xi, &yi) != 2) {
                free(poly);
                return 0; /* undefined per spec on malformed input; exit */
            }
            poly[i].x = (double)xi;
            poly[i].y = (double)yi;
        }

        /* Start with fixed square [0,10000] x [0,10000] as a convex polygon (CCW). */
        Pt cur[64], nxt[64];
        int cur_n = 4;
        cur[0].x = 0.0;     cur[0].y = 0.0;
        cur[1].x = 10000.0; cur[1].y = 0.0;
        cur[2].x = 10000.0; cur[2].y = 10000.0;
        cur[3].x = 0.0;     cur[3].y = 10000.0;

        int empty = 0;

        for (int i = 0; i < n; i++) {
            Pt a = poly[i];
            Pt b = poly[(i + 1) % n];

            cur_n = clip_halfplane(cur, cur_n, nxt, a, b);
            if (cur_n <= 0) {
                empty = 1;
                break;
            }

            /* Copy nxt -> cur */
            if (cur_n > (int)(sizeof(cur) / sizeof(cur[0]))) {
                /* Should not happen under stated constraints; keep narrow behavior: treat as empty. */
                empty = 1;
                break;
            }
            for (int k = 0; k < cur_n; k++) cur[k] = nxt[k];
        }

        printf("%d\n", empty ? 0 : 1);

        free(poly);
    }
    return 0;
}
