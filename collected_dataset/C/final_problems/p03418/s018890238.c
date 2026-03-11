#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#include<limits.h>

#define rep(i,begin,end) for(int i=begin; i<end; i++)
#define revrep(i,begin,end) for(int i=begin; i>end; i--)
#define lld long long int

int main(){
    lld n, k;
    scanf("%lld %lld", &n, &k);
    lld ans = 0;
    rep(i, (int)k+1, (int)n+1){
        ans += n/((lld)i) * ((lld)i - k);
        if(n%((lld)i) >= k){
            ans += (n%((lld)i)) - k + 1;
        }
    }
    if(k==0){
        ans -= n;
    }
    printf("%lld", ans);

    return 0;
}