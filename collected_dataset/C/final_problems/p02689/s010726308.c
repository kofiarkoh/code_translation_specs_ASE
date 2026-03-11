#include <stdio.h>

int meumax(long long a, long long b) {
    return (a > b) ? a : b;
}

int main() {
    long long n, k, h, a, b, c = 0;
    int obs[1000000], vc[1000000];
    
    scanf("%lld%lld", &n, &k);
    
    for (int i = 0; i < n; ++i)
        scanf("%ld", &obs[i]);
    
    for (int i = 0; i < k; ++i)
        vc[i] = 0;
    
    for (int i = 0; i < k; ++i) {
        scanf("%lld%lld", &a, &b);
        vc[meumax(a, b) - 1] = 1;
    }

    for (int i = 0; i <= k; ++i)
        if (vc[i] == 1)
            ++c;
    
    printf("%lld", c);

    return 0;
}