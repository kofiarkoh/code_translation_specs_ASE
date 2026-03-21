#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

#define MAXN 100000
#define PRIME_MAX 2221
#define THRESH 10000000000LL /* 1e10 */

static int primes[4000];
static int prime_cnt = 0;

static void gen_primes(void) {
    int is_comp[PRIME_MAX + 1];
    memset(is_comp, 0, sizeof(is_comp));
    for (int i = 2; i <= PRIME_MAX; i++) {
        if (!is_comp[i]) {
            primes[prime_cnt++] = i;
            if ((long long)i * i <= PRIME_MAX) {
                for (int j = i * i; j <= PRIME_MAX; j += i) is_comp[j] = 1;
            }
        }
    }
}

/* Integer sqrt via binary search; returns floor(sqrt(x)) for x>=0 */
static long long isqrt_ll(long long x) {
    long long lo = 0, hi = 3037000499LL; /* floor(sqrt(LLONG_MAX)) */
    if (x < 0) return 0;
    while (lo < hi) {
        long long mid = lo + (hi - lo + 1) / 2;
        __int128 sq = (__int128)mid * mid;
        if (sq <= x) lo = mid;
        else hi = mid - 1;
    }
    return lo;
}

static int is_perfect_square_ll(long long x) {
    if (x < 0) return 0;
    long long r = isqrt_ll(x);
    return r >= 0 && (__int128)r * r == x;
}

/* Remove all p^3 factors for primes <= 2221 by repeated division */
static long long cube_reduce(long long x) {
    long long t = x;
    for (int i = 0; i < prime_cnt; i++) {
        long long p = primes[i];
        long long p3 = p * p * p;
        while (t % p3 == 0) t /= p3;
    }
    return t;
}

/*
  Complement construction for v != 1.
  Returns 1 on success and sets *out_g.
  Returns 0 on failure (threshold exceeded or remaining-factor "unclassifiable" condition).
*/
static int build_complement(long long v, long long *out_g) {
    long long t = v;
    long long g = 1;

    for (int i = 0; i < prime_cnt; i++) {
        long long p = primes[i];
        int e = 0;
        while (t % p == 0) {
            t /= p;
            e++;
        }
        int r = e % 3;
        if (r != 0) {
            int need = 3 - r; /* 1->2, 2->1 */
            for (int k = 0; k < need; k++) {
                if (g > THRESH / p) return 0; /* threshold failure */
                g *= p;
            }
        }
    }

    if (t != 1) {
        if (t > 100000 && !is_perfect_square_ll(t)) {
            /* remaining factor treated as unclassifiable for complement pairing */
            return 0;
        }
        if (is_perfect_square_ll(t)) {
            long long q = isqrt_ll(t);
            /* t = q^2, exponent 2 => need one more q */
            if (q != 1) {
                if (g > THRESH / q) return 0;
                g *= q;
            }
        } else {
            /* t is treated as prime (or <=100000 composite but accepted by spec's narrow logic) */
            if (t != 1) {
                if (g > THRESH / t) return 0;
                g *= t;
                if (g > THRESH / t) return 0;
                g *= t; /* need exponent 2 */
            }
        }
    }

    *out_g = g;
    return 1;
}

static int cmp_ll(const void *a, const void *b) {
    long long x = *(const long long *)a;
    long long y = *(const long long *)b;
    return (x < y) ? -1 : (x > y);
}

/* lower_bound on sorted array of distinct values */
static int lower_bound_ll(const long long *arr, int n, long long key) {
    int lo = 0, hi = n;
    while (lo < hi) {
        int mid = lo + (hi - lo) / 2;
        if (arr[mid] < key) lo = mid + 1;
        else hi = mid;
    }
    return lo;
}

int main(void) {
    gen_primes();

    int n;
    if (scanf("%d", &n) != 1) return 0;

    long long stored[MAXN];
    int stored_n = 0;
    long long R = 0;

    for (int i = 0; i < n; i++) {
        long long x;
        if (scanf("%lld", &x) != 1) return 0;

        long long w = cube_reduce(x);

        /* Classification failure check uses remaining factor after removing primes<=2221 from w */
        long long t = w;
        for (int j = 0; j < prime_cnt; j++) {
            long long p = primes[j];
            while (t % p == 0) t /= p;
        }
        if (t > 100000 && !is_perfect_square_ll(t)) {
            /* unclassifiable: contributes 1 directly, not stored */
            R += 1;
        } else {
            if (stored_n < MAXN) stored[stored_n++] = w;
            /* else undefined per spec; we do nothing special */
        }
    }

    if (stored_n == 0) {
        printf("%lld\n", R);
        return 0;
    }

    qsort(stored, stored_n, sizeof(long long), cmp_ll);

    /* compress into groups */
    long long vals[MAXN];
    int cnts[MAXN];
    int m = 0;
    for (int i = 0; i < stored_n; ) {
        int j = i + 1;
        while (j < stored_n && stored[j] == stored[i]) j++;
        vals[m] = stored[i];
        cnts[m] = j - i;
        m++;
        i = j;
    }

    unsigned char processed[MAXN];
    memset(processed, 0, (size_t)m);

    for (int i = 0; i < m; i++) {
        if (processed[i]) continue;

        long long v = vals[i];
        int cv = cnts[i];

        if (v == 1) {
            R += 1;
            processed[i] = 1;
            continue;
        }

        long long g;
        if (!build_complement(v, &g)) {
            R += cv;
            processed[i] = 1;
            continue;
        }

        int pos = lower_bound_ll(vals, m, g);
        if (pos >= m || vals[pos] != g) {
            R += cv;
            processed[i] = 1;
            continue;
        }

        if (pos == i) {
            /* g(v) == v: no special rule beyond general logic; count all occurrences */
            R += cv;
            processed[i] = 1;
        } else {
            int cg = cnts[pos];
            R += (cv > cg) ? cv : cg;
            processed[i] = 1;
            processed[pos] = 1;
        }
    }

    printf("%lld\n", R);
    return 0;
}
