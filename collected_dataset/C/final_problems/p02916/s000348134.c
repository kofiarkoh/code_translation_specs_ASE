#include <stdio.h>

int main(void)
{
    int N, A[20], B[20], C[20] = {};
    int ans = 0;

    scanf("%d", &N);
    for (size_t i = 0; i < N; i++) {
        scanf("%d", &A[i]);
    }
    for (size_t i = 0; i < N; i++) {
        scanf("%d", &B[i]);
    }
    for (size_t i = 0; i < N; i++) {
        scanf("%d", &C[i]);
    }

    for (size_t i = 0; i < N; i++) {
        ans += B[A[i] - 1];
        if (((i + 1) < N) && (A[i] == (A[i + 1] - 1))) {
            ans += C[A[i] - 1];
        }
    }

    printf("%d\n", ans);

    return 0;
}