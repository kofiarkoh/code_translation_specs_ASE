#include <stdio.h>

int main(void) {
    int N;
    if (scanf("%d", &N) != 1) {
        return 0; /* unspecified behavior on input failure; exit quietly */
    }

    static long long A[100000 + 5];
    for (int i = 1; i <= N; i++) {
        if (scanf("%lld", &A[i]) != 1) {
            return 0; /* unspecified behavior on input failure; exit quietly */
        }
    }

    long long ans = 0;

    for (int i = 1; i <= N - 1; i++) {
        /* Internal pairing at i */
        ans += A[i] / 2;
        A[i] %= 2;

        /* Adjacent pairing between i and i+1 */
        if (A[i] == 1 && A[i + 1] > 1) {
            ans += 1;
            A[i] = 0;
            A[i + 1] -= 1;
        }
    }

    /* Final position internal pairing (after any decrements from left neighbor) */
    if (N >= 1) {
        ans += A[N] / 2;
    }

    printf("%lld\n", ans);
    return 0;
}
