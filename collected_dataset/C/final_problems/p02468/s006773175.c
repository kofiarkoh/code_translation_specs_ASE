#include <stdio.h>


int main() {
    int m, n, i;
    unsigned long long x;

    scanf("%d %d", &m, &n);

    x = 1;
    for (i = 0; i < n; i++) {
        x = (x * m) % 1000000007;
    }

    printf("%llu\n", x);

    return 0;
}