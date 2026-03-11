#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#include<limits.h>

#define rep(i,begin,end) for(int i=begin; i<end; i++)
#define lld long long int

#define MOD 1000000007

int main(){
    lld n;
    scanf("%lld", &n);
    lld a[1001];
    rep(i, 0, 1001){
        a[i] = 0;
    }
    lld ans = 1;
    for(lld i=2; i<=n; i++){
        lld x = i;
        while(x%2 == 0){
            a[2]++;
            x = x/2;
        }
        for(lld j=3; j*j<=x; j++){
            while(x % j == 0){
                a[j]++;
                x = x/j;
            }
        }
        if(x != 1){
            a[x]++;
        }
    }
    rep(i, 2, n+1){
        if(a[i] > 0){
            ans *= (a[i] + 1);
            ans = ans%MOD;
        }
    }
    printf("%lld", ans);

    return 0;
}