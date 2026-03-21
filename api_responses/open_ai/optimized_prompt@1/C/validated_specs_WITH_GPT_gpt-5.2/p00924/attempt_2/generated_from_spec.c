#include <stdio.h>

#define MAXN 14
#define MAXM 14

static long long compute_cost(int N, int M, const char inbits[], const int p[], char start_bit) {
    char s[MAXN];
    int i;

    for (i = 0; i < N; i++) s[i] = inbits[i];

    if (N == 1) return 0;

    /* Determine required starting bit for position 0 */
    char need = start_bit;

    if (N == 2) {
        /* As specified: cost depends only on whether first bit matches starting bit */
        return (s[0] == need) ? 0 : 1;
    }

    long long cost = 0;
    int run = 0;
    int rem = (M > 0) ? p[0] : 0;

    for (i = 0; i < N; i++) {
        /* Skip zero-length runs; they still toggle the required bit */
        while (run < M && rem == 0) {
            run++;
            need = (need == '0') ? '1' : '0';
            rem = (run < M) ? p[run] : 0;
        }

        /* If runs are exhausted early, behavior is undefined by spec; proceed with current need/rem. */

        if (s[i] != need) {
            int j = i;
            while (j < N && s[j] != need) j++;
            /* If not found, behavior is undefined by spec; this loop would run off. */
            cost += (long long)(j - i);

            /* Bring-forward by shifting s[i..j-1] right by one, placing s[j] at s[i] */
            char tmp = s[j];
            int k;
            for (k = j; k > i; k--) s[k] = s[k - 1];
            s[i] = tmp;
        }

        rem--;
    }

    return cost;
}

int main(void) {
    int N, M;
    if (scanf("%d %d", &N, &M) != 2) return 0;

    char bits[MAXN];
    int i;
    for (i = 0; i < N; i++) {
        char c;
        if (scanf(" %c", &c) != 1) return 0;
        bits[i] = c;
    }

    int p[MAXM];
    for (i = 0; i < M; i++) {
        if (scanf("%d", &p[i]) != 1) return 0;
    }

    int c1 = 0;
    for (i = 0; i < N; i++) if (bits[i] == '1') c1++;

    int ce = 0;
    for (i = 0; i < M; i += 2) ce += p[i];

    long long ans;
    if (c1 * 2 == N) {
        long long c_start1 = compute_cost(N, M, bits, p, '1');
        long long c_start0 = compute_cost(N, M, bits, p, '0');
        ans = (c_start1 < c_start0) ? c_start1 : c_start0;
    } else {
        char start = (ce == c1) ? '1' : '0';
        ans = compute_cost(N, M, bits, p, start);
    }

    printf("%lld\n", ans);
    return 0;
}
