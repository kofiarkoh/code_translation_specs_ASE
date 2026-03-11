#include <stdio.h>

long long int modpos(long long int m, long long int n, long long int d) {
    long long int res;
    if(n == 0) return 1;
   
    res = modpos((m%d)*(m%d), n / 2, d);
    if(n % 2) {
        res = res * (m%d);
    }
    return res;
}

int main(void) {
    long long int m, n;
    long long int d = 1000000007ll;
   
    scanf("%lld %lld\n", &m, &n);
    printf("%lld\n",modpos(m, n ,d));
   
    return 0;
}