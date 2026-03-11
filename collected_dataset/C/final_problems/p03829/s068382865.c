#include<stdio.h>

long long N,A,B,X[100001];

int main(void){
    int i,j,k;
    long long ans;

    scanf("%lld %lld %lld",&N,&A,&B);

    for(i = 1;i <= N;i++)
        scanf("%lld",&X[i]);
    ans = 0;k = N - 1;

    for(i = 1;i < N;i++){
        if((X[i + 1] - X[i]) * A < B){
            ans += (X[i + 1] - X[i]) * A;
            k--;
        }
    }

    ans += k * B;

    printf("%lld\n",ans);
    return 0;
}