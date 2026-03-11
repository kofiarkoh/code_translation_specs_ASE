#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int min(size_t a, size_t b)
{
    if(a < b){
        return a;
    }
    return b;
}

int main()
{
    long long int n, m, k;
    long long int i;
    long long int a[200000], b[200000];
    long long int ans = 0;
    a[0] = b[0] = 0;

    scanf("%lld %lld %lld", &n, &m, &k);
    for(i = 0; i < n; i++){
        scanf("%lld", &a[i + 1]);
    }
    for(i = 0; i < m; i++){
        scanf("%lld", &b[i + 1]);
    }

    for(i = 0; i < n; i++){
        a[i + 1] += a[i];
    }
    for(i = 0; i < m; i++){
        b[i + 1] += b[i];
    }

    for(i = 0; i <= n; i++){
        while(m >= 0 && a[i] + b[m] > k){
            m--;
        }
        if(m >= 0){
            if (ans < i + m){
                ans = i + m;
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}