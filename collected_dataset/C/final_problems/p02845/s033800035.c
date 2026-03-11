#include <stdio.h>

int main(void) {
    long long N, A[200005], B[10], X, K, i, j;
    scanf("%lld", &N);
    for (i=1; i<=N; i++) scanf("%lld", &A[i]);
    for (i=1; i<=3; i++) B[i]=0;
    X=1;
    for (i=1; i<=N; i++) {
        K=0;
        for (j=1; j<=3; j++) {
            if (B[j]==A[i]) {
                if (K==0) B[j]++;
                K++;
            }
        }
        if (K==3) X*=3;
        else if (K==2) X*=2;
        if (X>=1000000007) X%=1000000007;
        // printf("%d: %d\n", i, X);
    }
    // printf("%lld %lld %lld\n", B[1], B[2], B[3]);
    if (X>=1000000007) X%=1000000007;
    printf("%lld", X); 
    return 0;
}