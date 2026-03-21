#include <stdio.h>
#include <string.h>

#define MAXN 14
#define MAXM 14

static int p[MAXM];
static char s0[MAXN + 1];

static char flip_bit(char c) {
    return (c == '0') ? '1' : '0';
}

/*
 * Recursive cost computation as specified.
 * Parameters:
 *  - s: pointer to current suffix (within a mutable buffer)
 *  - n: length of current suffix
 *  - idx: current run index into p[]
 *  - rem: remaining length in current run
 *  - need: currently required bit ('0' or '1')
 *
 * Undefined behavior is preserved as much as possible:
 *  - If idx runs past M, we still read p[idx] (out of bounds) if the logic reaches it.
 *  - If the required bit does not exist in the suffix, the search will run past bounds.
 */
static int cost_rec(char *s, int n, int idx, int rem, char need, int M) {
    (void)M; /* M is not used for bounds checks by design */

    if (n == 1) return 0;

    if (n == 2) {
        return (s[0] == need) ? 0 : 1;
    }

    if (rem == 0) {
        idx++;
        need = flip_bit(need);
        rem = p[idx]; /* may be out of bounds: intentionally no check */
        return cost_rec(s, n, idx, rem, need, M);
    }

    if (s[0] == need) {
        return cost_rec(s + 1, n - 1, idx, rem - 1, need, M);
    } else {
        int pos = 0;
        while (s[pos] != need) {
            pos++; /* may run past n: intentionally no check */
        }
        /* mutate found character by flipping it */
        s[pos] = flip_bit(s[pos]);
        return pos + cost_rec(s + 1, n - 1, idx, rem - 1, need, M);
    }
}

static int compute_cost_for_start(const char *orig, int N, int M, char start) {
    char buf[MAXN + 1];
    int idx = 0;
    int rem;

    memcpy(buf, orig, (size_t)N);
    buf[N] = '\0';

    rem = p[idx];

    /* Skip leading zero-length runs; skipping toggles required bit */
    while (rem == 0) {
        idx++;
        start = flip_bit(start);
        rem = p[idx]; /* may be out of bounds: intentionally no check */
    }

    return cost_rec(buf, N, idx, rem, start, M);
}

int main(void) {
    int N, M;
    int i;
    int c1 = 0;
    int ce = 0;

    if (scanf("%d %d", &N, &M) != 2) return 0;

    /* Read N bit characters with whitespace skipping */
    for (i = 0; i < N; i++) {
        char c;
        if (scanf(" %c", &c) != 1) return 0;
        s0[i] = c;
        if (c == '1') c1++;
    }
    s0[N] = '\0';

    for (i = 0; i < M; i++) {
        if (scanf("%d", &p[i]) != 1) return 0;
        if ((i % 2) == 0) ce += p[i];
    }

    if (c1 * 2 == N) {
        int a = compute_cost_for_start(s0, N, M, '1');
        int b = compute_cost_for_start(s0, N, M, '0');
        printf("%d\n", (a < b) ? a : b);
    } else {
        char start = (ce == c1) ? '1' : '0';
        int ans = compute_cost_for_start(s0, N, M, start);
        printf("%d\n", ans);
    }

    return 0;
}
