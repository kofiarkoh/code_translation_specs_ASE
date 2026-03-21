#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 1000000007

static inline int addmod(int a, int b) {
    int s = a + b;
    if (s >= MOD) s -= MOD;
    return s;
}

static inline int submod(int a, int b) {
    int s = a - b;
    if (s < 0) s += MOD;
    return s;
}

static inline int mulmod(long long a, long long b) {
    return (int)((a * b) % MOD);
}

/* Build KMP prefix function (pi) for pattern P of length M. */
static void build_pi(const char *P, int M, int *pi) {
    pi[0] = 0;
    int j = 0;
    for (int i = 1; i < M; i++) {
        while (j > 0 && P[i] != P[j]) j = pi[j - 1];
        if (P[i] == P[j]) j++;
        pi[i] = j;
    }
}

/* Build automaton: next_state[q][d] for q in [0..M], d in [0..9].
   State q means current matched prefix length q (0..M). State M is absorbing. */
static void build_automaton(const char *P, int M, const int *pi, int next_state[][10]) {
    for (int q = 0; q <= M; q++) {
        for (int d = 0; d <= 9; d++) {
            if (q == M) {
                next_state[q][d] = M;
                continue;
            }
            char c = (char)('0' + d);
            int j = q;
            while (j > 0 && P[j] != c) j = pi[j - 1];
            if (P[j] == c) j++;
            if (j == M) next_state[q][d] = M;
            else next_state[q][d] = j;
        }
    }
}

/* Count numbers in [0..bound] (inclusive), where bound is an N-digit string (leading zeros allowed),
   such that pattern occurs as a substring starting at some index s >= minStart.
   minStart is 0-based index in [0..N], and start positions are 0..N-M.
   If M > N, result is 0. */
static int count_upto_bound(const char *bound, int N,
                            const int next_state[][10], int M,
                            int minStart) {
    if (M > N) return 0;

    /* DP over positions with tight flag and KMP state.
       dp_tight[state], dp_loose[state] for current position. */
    int *dpT = (int*)calloc((size_t)(M + 1), sizeof(int));
    int *dpL = (int*)calloc((size_t)(M + 1), sizeof(int));
    int *ndpT = (int*)calloc((size_t)(M + 1), sizeof(int));
    int *ndpL = (int*)calloc((size_t)(M + 1), sizeof(int));
    if (!dpT || !dpL || !ndpT || !ndpL) {
        /* Undefined by spec; exit non-zero to avoid silent wrong output. */
        exit(2);
    }

    dpT[0] = 1;

    for (int i = 0; i < N; i++) {
        memset(ndpT, 0, (size_t)(M + 1) * sizeof(int));
        memset(ndpL, 0, (size_t)(M + 1) * sizeof(int));

        int limit = bound[i] - '0';

        /* From tight states */
        for (int q = 0; q <= M; q++) {
            int ways = dpT[q];
            if (!ways) continue;

            for (int d = 0; d <= limit; d++) {
                int ntight = (d == limit);
                int nq = q;

                /* Enforce "match start must be >= minStart":
                   If we are before minStart, we must not allow reaching state M.
                   Reaching M at position i means a match ending at i, starting at i-M+1.
                   Condition start >= minStart <=> i - M + 1 >= minStart <=> i >= minStart + M - 1.
                   So if i < minStart + M - 1, disallow transitions that would reach M. */
                int cand = next_state[q][d];
                if (cand == M && i < (minStart + M - 1)) {
                    continue;
                }
                nq = cand;

                if (ntight) ndpT[nq] = addmod(ndpT[nq], ways);
                else ndpL[nq] = addmod(ndpL[nq], ways);
            }
        }

        /* From loose states */
        for (int q = 0; q <= M; q++) {
            int ways = dpL[q];
            if (!ways) continue;

            for (int d = 0; d <= 9; d++) {
                int cand = next_state[q][d];
                if (cand == M && i < (minStart + M - 1)) {
                    continue;
                }
                ndpL[cand] = addmod(ndpL[cand], ways);
            }
        }

        /* swap */
        int *tmp;
        tmp = dpT; dpT = ndpT; ndpT = tmp;
        tmp = dpL; dpL = ndpL; ndpL = tmp;
    }

    /* Accept states where pattern has occurred: KMP state == M. */
    int ans = addmod(dpT[M], dpL[M]);

    free(dpT);
    free(dpL);
    free(ndpT);
    free(ndpL);
    return ans;
}

/* Decrement an N-digit decimal string in-place: s = s - 1.
   Returns 1 if successful (i.e., original s > 0), 0 if underflow (original s == 0...0). */
static int dec_string(char *s, int N) {
    for (int i = N - 1; i >= 0; i--) {
        if (s[i] > '0') {
            s[i]--;
            return 1;
        }
        s[i] = '9';
    }
    return 0;
}

int main(void) {
    char *low = NULL, *up = NULL, *pat = NULL;

    /* Read three non-empty digit strings. */
    size_t cap = 0;
    ssize_t r;

    /* Use getline for robustness with large N; if unavailable, this may fail to compile on some systems.
       The spec does not constrain the environment; we keep it simple with scanf-like fallback if needed. */
#if defined(_GNU_SOURCE) || defined(__APPLE__)
    r = getline(&low, &cap, stdin);
    if (r <= 0) return 1;
    cap = 0;
    r = getline(&up, &cap, stdin);
    if (r <= 0) return 1;
    cap = 0;
    r = getline(&pat, &cap, stdin);
    if (r <= 0) return 1;

    /* Strip trailing newlines. */
    for (char *p = low; *p; p++) if (*p == '\n' || *p == '\r') { *p = '\0'; break; }
    for (char *p = up; *p; p++) if (*p == '\n' || *p == '\r') { *p = '\0'; break; }
    for (char *p = pat; *p; p++) if (*p == '\n' || *p == '\r') { *p = '\0'; break; }
#else
    /* Fallback: fixed buffers (may violate spec max sizes if too small). */
    char lowbuf[10050], upbuf[10050], patbuf[600];
    if (scanf("%10049s %10049s %599s", lowbuf, upbuf, patbuf) != 3) return 1;
    low = strdup(lowbuf);
    up = strdup(upbuf);
    pat = strdup(patbuf);
    if (!low || !up || !pat) return 1;
#endif

    int N = (int)strlen(up);
    int L0 = (int)strlen(low);
    int M = (int)strlen(pat);

    /* Left-pad lower bound to length N. */
    char *lowpad = (char*)malloc((size_t)N + 1);
    if (!lowpad) return 1;
    int pad = N - L0; /* >= 0 by precondition */
    for (int i = 0; i < pad; i++) lowpad[i] = '0';
    memcpy(lowpad + pad, low, (size_t)L0);
    lowpad[N] = '\0';

    /* minStart is the first index where padded zeros may appear: N - original_lower_length. */
    int minStart = pad;

    /* Build KMP automaton. */
    int *pi = (int*)malloc((size_t)M * sizeof(int));
    if (!pi) return 1;

    /* next_state has (M+1) x 10; M <= 500 so stack is fine, but use VLA for strictness. */
    int (*next_state)[10] = (int (*)[10])malloc((size_t)(M + 1) * 10 * sizeof(int));
    if (!next_state) return 1;

    build_pi(pat, M, pi);
    build_automaton(pat, M, pi, next_state);

    /* Count in [lowpad..up] = count_upto(up) - count_upto(lowpad-1). */
    int cnt_up = count_upto_bound(up, N, next_state, M, minStart);

    char *lowm1 = (char*)malloc((size_t)N + 1);
    if (!lowm1) return 1;
    memcpy(lowm1, lowpad, (size_t)N + 1);

    int cnt_lowm1 = 0;
    if (dec_string(lowm1, N)) {
        cnt_lowm1 = count_upto_bound(lowm1, N, next_state, M, minStart);
    } else {
        cnt_lowm1 = 0;
    }

    int ans = submod(cnt_up, cnt_lowm1);
    printf("%d\n", ans);

    free(low);
    free(up);
    free(pat);
    free(lowpad);
    free(lowm1);
    free(pi);
    free(next_state);
    return 0;
}
