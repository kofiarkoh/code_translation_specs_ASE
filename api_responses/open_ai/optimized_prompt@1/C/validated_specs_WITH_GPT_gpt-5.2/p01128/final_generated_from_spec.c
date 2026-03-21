#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define MAXN 100000

typedef struct {
    int64_t x, y;
} Pt;

typedef struct {
    /* Sort key: position along reference segment, represented as a rational t = num/den.
       We store num = cross((C-A), s) and den = cross(r, s), with den > 0 normalized. */
    int64_t num;
    int64_t den;
    int type;
    int label; /* intended 0/1 */
} Event;

static int64_t cross2(Pt a, Pt b) {
    return a.x * b.y - a.y * b.x;
}

static Pt sub(Pt a, Pt b) {
    Pt r;
    r.x = a.x - b.x;
    r.y = a.y - b.y;
    return r;
}

/* Compare two rationals a.num/a.den and b.num/b.den without floating point.
   Assumes den > 0 for both. */
static int cmp_event(const void *pa, const void *pb) {
    const Event *a = (const Event *)pa;
    const Event *b = (const Event *)pb;

    __int128 lhs = (__int128)a->num * (__int128)b->den;
    __int128 rhs = (__int128)b->num * (__int128)a->den;
    if (lhs < rhs) return -1;
    if (lhs > rhs) return 1;

    /* Tie-breaker: arbitrary but consistent. */
    if (a->type != b->type) return (a->type < b->type) ? -1 : 1;
    if (a->label != b->label) return (a->label < b->label) ? -1 : 1;
    return 0;
}

/* Proper interior intersection test between segments AB and CD, excluding endpoints.
   Also ignores parallel/collinear (cross(r,s)==0) as specified.
   If proper intersection exists, fills out t = num/den along AB (0<t<1) and returns 1. */
static int proper_intersection_t(Pt A, Pt B, Pt C, Pt D, int64_t *out_num, int64_t *out_den) {
    Pt r = sub(B, A);
    Pt s = sub(D, C);

    int64_t den = cross2(r, s);
    if (den == 0) {
        /* parallel or collinear: ignored */
        return 0;
    }

    Pt CA = sub(C, A);
    int64_t t_num = cross2(CA, s);
    int64_t u_num = cross2(CA, r);

    /* We need 0 < t < 1 and 0 < u < 1, with t = t_num/den, u = u_num/den.
       Handle sign of den carefully to avoid division and to enforce strict interior. */
    if (den > 0) {
        if (!(t_num > 0 && t_num < den)) return 0;
        if (!(u_num > 0 && u_num < den)) return 0;
        *out_num = t_num;
        *out_den = den;
        return 1;
    } else {
        /* den < 0: inequalities flip when multiplying by den */
        if (!(t_num < 0 && t_num > den)) return 0;
        if (!(u_num < 0 && u_num > den)) return 0;
        *out_num = -t_num;
        *out_den = -den; /* normalize to positive */
        return 1;
    }
}

/* Constraint satisfaction:
   Specification mentions a "distinguished type value treated specially" but does not
   define which value nor the rule. Narrowest reasonable interpretation:
   - If type == 0 (distinguished), constraint is satisfied when state == label.
   - Otherwise, constraint is satisfied when state != label.
   This uses both type and label and yields a toggle/no-toggle decision. */
static int constraint_satisfied(int state, int type, int label) {
    label &= 1;
    state &= 1;
    if (type == 0) return state == label;
    return state != label;
}

int main(void) {
    int T;
    if (scanf("%d", &T) != 1) return 0;

    while (T--) {
        Pt A, B;
        int n;

        if (scanf("%lld %lld %lld %lld", (long long *)&A.x, (long long *)&A.y,
                  (long long *)&B.x, (long long *)&B.y) != 4) {
            return 0;
        }
        if (scanf("%d", &n) != 1) return 0;

        static Event ev[MAXN + 5];
        int m = 0;

        for (int i = 0; i < n; i++) {
            Pt C, D;
            int type, label;
            if (scanf("%lld %lld %lld %lld %d %d",
                      (long long *)&C.x, (long long *)&C.y,
                      (long long *)&D.x, (long long *)&D.y,
                      &type, &label) != 6) {
                return 0;
            }

            if (m >= MAXN) {
                /* Undefined behavior per spec if capacity exceeded; we just stop storing. */
                continue;
            }

            int64_t num, den;
            if (proper_intersection_t(A, B, C, D, &num, &den)) {
                ev[m].num = num;
                ev[m].den = den;
                ev[m].type = type;
                ev[m].label = label & 1;
                m++;
            }
        }

        qsort(ev, (size_t)m, sizeof(Event), cmp_event);

        int best = -1;
        for (int init = 0; init <= 1; init++) {
            int state = init;
            int toggles = 0;
            for (int i = 0; i < m; i++) {
                if (!constraint_satisfied(state, ev[i].type, ev[i].label)) {
                    state ^= 1;
                    toggles++;
                }
            }
            if (best < 0 || toggles < best) best = toggles;
        }

        if (best < 0) best = 0;
        printf("%d\n", best);
    }

    return 0;
}
