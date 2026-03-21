#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define CATS 26

static long long compute_score(int D, const int *c, const int s[][CATS], const int *penalty) {
    /* Score(schedule):
       For each day d=1..D:
         add satisfaction s[d][c[d]]
         subtract sum_{i=1..26} penalty[i] * (d - last[i])
         then set last[c[d]] = d
       with last[i] initially 0.
    */
    int last[CATS];
    for (int i = 0; i < CATS; i++) last[i] = 0;

    long long score = 0;
    for (int d = 1; d <= D; d++) {
        int cat = c[d - 1]; /* stored as 1..26 */
        score += (long long)s[d - 1][cat - 1];

        for (int i = 0; i < CATS; i++) {
            score -= (long long)penalty[i] * (long long)(d - last[i]);
        }
        last[cat - 1] = d;
    }
    return score;
}

static unsigned long long rng_state = 88172645463325252ULL;
static unsigned long long xorshift64star(void) {
    /* Simple PRNG; spec only requires randomized behavior, not a specific RNG. */
    unsigned long long x = rng_state;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    rng_state = x;
    return x * 2685821657736338717ULL;
}

static int rand_int(int n) {
    /* Returns integer in [0, n-1], undefined if n<=0 (consistent with spec's undefined behavior on bad inputs). */
    return (int)(xorshift64star() % (unsigned long long)n);
}

int main(void) {
    int D;
    if (scanf("%d", &D) != 1) return 0;

    int penalty[CATS];
    for (int i = 0; i < CATS; i++) {
        (void)scanf("%d", &penalty[i]);
    }

    /* Stack allocation as per spec; if too large, behavior is undefined. */
    int (*s)[CATS] = (int (*)[CATS])alloca((size_t)D * sizeof(*s));
    for (int d = 0; d < D; d++) {
        for (int i = 0; i < CATS; i++) {
            (void)scanf("%d", &s[d][i]);
        }
    }

    /* Seed RNG with time; output is intentionally non-deterministic across runs. */
    rng_state ^= (unsigned long long)time(NULL);
    rng_state ^= (unsigned long long)clock();

    /* Initialization: for each day independently choose category with highest satisfaction.
       Precondition: at least one strictly positive satisfaction per day; otherwise undefined. */
    int *c = (int *)alloca((size_t)D * sizeof(int));
    for (int d = 0; d < D; d++) {
        int best_i = 0;
        int best_v = s[d][0];
        for (int i = 1; i < CATS; i++) {
            if (s[d][i] > best_v) {
                best_v = s[d][i];
                best_i = i;
            }
        }
        c[d] = best_i + 1; /* 1..26 */
    }

    long long best_score = compute_score(D, c, s, penalty);

    /* Time-bounded improvement phase (~2 seconds of processor time). */
    const double TIME_LIMIT_SEC = 2.0;
    clock_t start = clock();

    while (((double)(clock() - start) / (double)CLOCKS_PER_SEC) < TIME_LIMIT_SEC) {
        if (D <= 1) break;

        int d1 = rand_int(D);
        int d2 = rand_int(D);
        if (d1 == d2) continue;

        int tmp = c[d1];
        c[d1] = c[d2];
        c[d2] = tmp;

        long long new_score = compute_score(D, c, s, penalty);
        if (new_score > best_score) {
            best_score = new_score; /* accepted swap; strictly improves */
        } else {
            /* reject swap: revert */
            tmp = c[d1];
            c[d1] = c[d2];
            c[d2] = tmp;
        }
    }

    for (int d = 0; d < D; d++) {
        printf("%d\n", c[d]);
    }
    return 0;
}
