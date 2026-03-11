#include <stdio.h>

#define N 200001
int r[2 * N] = {0};

int main() {
    int n, s = 0, a[N], t;
    long long int u;
    scanf("%d", &n);
    for (int i = 1; i <= n; i++) {
        scanf("%lld", &u);
        if (u < N) {
            t = i - u;
            if (0 <= t && t < 2 * N) r[t]++;
            a[i] = (int) u;
        } else a[i] = 0;
    }
    for (int i = 1; i < n; i++) {
        u = a[i] + i;
        if (a[i]) {
            t = i - a[i];
            if (0 <= t) r[t]--;
            if (u < 2 * N) s += r[u];
        }
    }
    printf("%d", s);
    return 0;
}