#include <stdio.h>
#include <stdint.h>

#define PEOPLE 8
#define MAXN 1000 /* Fixed-size internal tables; behavior undefined if input exceeds this. */

static int64_t memo[MAXN + 1][PEOPLE + 1];
static unsigned char seen[MAXN + 1][PEOPLE + 1];

static void reset_memo(void) {
    for (int i = 0; i <= MAXN; i++) {
        for (int j = 0; j <= PEOPLE; j++) {
            seen[i][j] = 0;
            memo[i][j] = 0;
        }
    }
}

/* Count ways to choose k non-adjacent seats on a line of length n (positions fixed). */
static int64_t line_choose(int n, int k) {
    if (k == 0) return 1;
    if (n <= 0) return 0;
    if (k < 0) return 0;
    if (n > MAXN) {
        /* Undefined per spec; we do not guard further. */
    }
    if (seen[n][k]) return memo[n][k];
    seen[n][k] = 1;

    /* Standard recurrence: f(n,k) = f(n-1,k) + f(n-2,k-1) */
    int64_t a = line_choose(n - 1, k);
    int64_t b = line_choose(n - 2, k - 1);
    memo[n][k] = a + b;
    return memo[n][k];
}

/* Count ways to choose 8 non-adjacent seats on a cycle of size n (positions fixed). */
static int64_t cycle_choose(int n) {
    /* For n <= 16, spec requires output 0; this function is used only for n > 16. */
    /* Cycle to line reduction:
       - If seat 1 not chosen: choose 8 on line of length n-1.
       - If seat 1 chosen: seats 2 and n forbidden; choose 7 on line of length n-3.
    */
    return line_choose(n - 1, PEOPLE) + line_choose(n - 3, PEOPLE - 1);
}

static int64_t factorial8(void) {
    /* 8! = 40320 */
    return 40320;
}

int main(void) {
    int n;
    while (scanf("%d", &n) == 1) {
        if (n == 0) break;

        reset_memo(); /* Spec: memoization state is reset between successive input values. */

        int64_t result = 0;
        if (n <= 16) {
            result = 0;
        } else {
            int64_t ways = cycle_choose(n);
            result = ways * factorial8(); /* May overflow int64_t per spec. */
        }

        printf("%lld\n", (long long)result);
    }
    return 0;
}
