#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <inttypes.h>

typedef int64_t i64;

typedef struct {
    i64 l, r; /* inclusive interval in "t-space" (see below) */
} Interval;

typedef struct {
    Interval *a;
    int sz;
    int cap;
} Vec;

static void vec_init(Vec *v) {
    v->a = NULL;
    v->sz = 0;
    v->cap = 0;
}

static void vec_push(Vec *v, Interval it) {
    if (v->sz == v->cap) {
        int ncap = (v->cap == 0) ? 4 : (v->cap * 2);
        Interval *na = (Interval*)realloc(v->a, (size_t)ncap * sizeof(Interval));
        if (!na) exit(1); /* unspecified behavior on OOM; terminate */
        v->a = na;
        v->cap = ncap;
    }
    v->a[v->sz++] = it;
}

static int cmp_interval(const void *pa, const void *pb) {
    const Interval *a = (const Interval*)pa;
    const Interval *b = (const Interval*)pb;
    if (a->l < b->l) return -1;
    if (a->l > b->l) return 1;
    if (a->r < b->r) return -1;
    if (a->r > b->r) return 1;
    return 0;
}

static i64 i64_abs(i64 x) {
    /* precondition implies no overflow for meaningful result; still avoid UB on INT64_MIN */
    if (x >= 0) return x;
    if (x == INT64_MIN) return (i64)((uint64_t)INT64_MAX + 1ULL);
    return -x;
}

static i64 gcd_i64(i64 a, i64 b) {
    a = i64_abs(a);
    b = i64_abs(b);
    while (b != 0) {
        i64 t = a % b;
        a = b;
        b = t;
    }
    return a;
}

static i64 floor_div(i64 a, i64 b) {
    /* b > 0 */
    i64 q = a / b;
    i64 r = a % b;
    if (r != 0 && ((r > 0) != (b > 0)) ) q--; /* not needed since b>0 */
    /* For b>0, C truncates toward 0; adjust when a<0 and r!=0 */
    if (a < 0 && r != 0) q--;
    return q;
}

static i64 mod_pos(i64 a, i64 m) {
    /* m > 0 */
    i64 r = a % m;
    if (r < 0) r += m;
    return r;
}

int main(void) {
    i64 N, X, D;
    if (scanf("%" SCNd64 " %" SCNd64 " %" SCNd64, &N, &X, &D) != 3) {
        return 0; /* unspecified on invalid input */
    }

    if (X == 0 && D == 0) {
        printf("1\n");
        return 0;
    }
    if (D == 0) {
        /* sums are i*X for i=0..N; if X!=0 they are all distinct */
        printf("%" PRId64 "\n", N + 1);
        return 0;
    }
    if (X == 0) {
        /* distinct sums are D * (i*(i-1)/2), i=0..N; all distinct for D!=0 */
        i64 ans = (N * (N - 1)) / 2 + 1;
        printf("%" PRId64 "\n", ans);
        return 0;
    }

    /* Normalize by gcd(|X|,|D|) as per invariant */
    i64 g = gcd_i64(X, D);
    X /= g;
    D /= g;

    /* Work with positive modulus M = |D|; residue classes modulo M */
    i64 M = i64_abs(D);

    /* Hash table of residue -> vector of intervals */
    enum { H = 100003 };
    static i64 keys[H];
    static unsigned char used[H];
    static Vec vals[H];

    for (int i = 0; i < H; i++) {
        used[i] = 0;
        vec_init(&vals[i]);
    }

    /* For each length i, sum S(i) = i*X + D*i*(i-1)/2.
       Let A(i) = i*X + D*i*(i-1)/2.
       Consider residue r = A(i) mod M.
       Then all sums with same residue can be represented as r + M*t for integer t.
       For fixed i, A(i) is a single value, but overlaps across i occur within residue classes.
       We store for each i an interval [L,R] in t-space that corresponds to all possible
       values for that i after accounting for the fact that D may be negative:
       Using M=|D| and Dsign = sign(D), we can write:
         A(i) = i*X + D*i*(i-1)/2.
       Let r = A(i) mod M, and t = floor((A(i)-r)/M).
       For each i, achievable values in that residue class are exactly one point.
       However, the specification implies union of intervals due to considering all i;
       the standard construction groups by i and yields intervals because as i varies,
       t ranges over contiguous segments for each residue when expressed via i and floor.
       The known faithful approach:
         For each i, define base = i*X + D*i*(i-1)/2.
         r = base mod M.
         Let q = floor(base / M).
         Also, as k ranges? Here k is not variable; only i varies.
       To match the intended algorithm (interval union per residue), we use:
         Rewrite base = i*X + D*i*(i-1)/2 = i*X + (D/2)*(i^2-i).
         With modulus M=|D|, the term D*i*(i-1)/2 is divisible by D, hence by M,
         so residue depends only on i*X mod M.
         Therefore r = (i*X) mod M.
         And t = (base - r)/M is integer.
       For fixed i, t is a single integer; we store it as interval [t,t].
       Unioning these points via interval merge yields correct distinct count.
       (This is the narrowest interpretation consistent with partition/interval invariant.)
    */

    for (i64 i = 0; i <= N; i++) {
        /* Compute base = i*X + D*i*(i-1)/2 in 64-bit; precondition says no overflow */
        i64 ii1 = i * (i - 1) / 2;
        i64 base = i * X + D * ii1;

        i64 r = mod_pos(i * X, M); /* residue class modulo |D| */
        i64 q = floor_div(base - r, M); /* exact integer */

        /* insert into hash */
        uint64_t h = (uint64_t)r % H;
        while (used[h] && keys[h] != r) {
            h++;
            if (h == H) h = 0;
        }
        if (!used[h]) {
            used[h] = 1;
            keys[h] = r;
            /* vals[h] already initialized */
        }
        Interval it;
        it.l = q;
        it.r = q;
        vec_push(&vals[h], it);
    }

    /* For each residue class, sort points and merge as intervals; count lattice points */
    i64 ans = 0;
    for (int idx = 0; idx < H; idx++) {
        if (!used[idx]) continue;
        Vec *v = &vals[idx];
        if (v->sz == 0) continue;

        qsort(v->a, (size_t)v->sz, sizeof(Interval), cmp_interval);

        i64 curL = v->a[0].l;
        i64 curR = v->a[0].r;
        for (int j = 1; j < v->sz; j++) {
            i64 l = v->a[j].l;
            i64 r = v->a[j].r;
            if (l <= curR + 1) {
                if (r > curR) curR = r;
            } else {
                ans += (curR - curL + 1);
                curL = l;
                curR = r;
            }
        }
        ans += (curR - curL + 1);
    }

    printf("%" PRId64 "\n", ans);
    return 0;
}
