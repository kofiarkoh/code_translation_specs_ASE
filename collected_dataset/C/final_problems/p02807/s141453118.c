#include <stdio.h>
#include <string.h>

long long f(long long a, long long n, long long mod) {
    long long res = 1;
    while (n > 0) {
        if (n & 1) res = res * a % mod;
        a = a * a % mod;
        n >>= 1;
    }
    return res;
}

long long g(long long a, long long mod) {
    return f(a, mod-2, mod);
}

int main(void) {
    long long N, x[100005], D[100005], X, a, i;
    long long MOD=1000000007;
    scanf("%lld", &N);
    for (i=1; i<=N; i++) scanf("%lld", &x[i]);
    
    D[1]=1;
    for (i=1; i<=N-1; i++) {
        D[1]*=i;
        if (D[1]>=MOD) D[1]%=MOD;
    }
    for (i=2; i<=N-1; i++) {
        a=g(i, MOD)%MOD;
        D[i]=D[1]*a%MOD;
    }
    D[N]=0;
    for (i=1; i<=N-1; i++) {
        D[N]+=D[i];
        if (D[N]>=MOD) D[N]%=MOD;
    }
    for (i=1; i<=N-1; i++) D[i]*=-1;
    
    X=0;
    if (X>=MOD) X%=MOD;
    for (i=1; i<=N; i++) {
        a=x[i]%MOD;
        X+=D[i]*a;
        if (X>=MOD) X%=MOD;
        // printf("%lld %lld\n", i, X);
    }
    if (X<0) X+=(MOD-7)*MOD;
    X%=MOD;
    // for (i=1; i<=N; i++) printf("%lld ", D[i]);
    // printf("\n");
    printf("%lld", X);
}