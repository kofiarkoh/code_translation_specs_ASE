#include <stdio.h>

int main(void)
{
    int n;
    scanf("%d", &n);

    int A[1000];
    int i;
    for (i = 0; i < n; i++) scanf("%d", A + i);

    int q;
    scanf("%d", &q);

    int b, m, e, k, mod;
    int T[1000];
    while (q--) {
        scanf("%d %d %d", &b, &m, &e);
        mod = e - b;
        for (k = 0; k < mod; k++) {
            T[b + ((k + e - m) % mod)] = A[b + k];
        }
        for (i = b; i < e; i++) {
            A[i] = T[i];
        }
    }

    n--;
    for (i = 0; i < n; i++) printf("%d ", A[i]);
    printf("%d\n", A[n]);

    return 0;
}
