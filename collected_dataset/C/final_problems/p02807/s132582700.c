#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#include<limits.h>

#define rep(i,begin,end) for(int i=begin; i<end; i++)
#define lld long long int

#define MOD 1000000007
#define MAX 100001
long long int fac[MAX], finv[MAX], inv[MAX];
void MOD_COM_init(){
    fac[0] = 1;
    fac[1] = 1;
    finv[0] = 1;
    finv[1] = 1;
    inv[1] = 1;
    for(int i=2; i<MAX; i++){
        fac[i] = fac[i-1]*i % MOD;
        inv[i] = MOD - inv[MOD%i] * (MOD/i) % MOD;
        finv[i] = finv[i-1] * inv[i] % MOD;
    }
}

int main(){
    lld n;
    scanf("%lld", &n);
    lld x[n];
    for(lld i=0; i<n; i++){
        scanf("%lld", &x[i]);
    }
    MOD_COM_init();
    lld ans = 0;
    for(lld i=0; i<=n-3; i++){
        lld q = 2, r=3, t=1;
        for(lld j=i+1; j<=n-1; j++){
            lld len = x[j] - x[i];
            if(j == n-1){
                ans += len * t % MOD;
                ans = ans % MOD;
                break;
            }
            lld ad = (len * ((t * inv[q])%MOD) %MOD);
            t = t - ((t * inv[q])%MOD);
            while(t < 0LL){
                t += MOD;
            }
            ans += ad;
            ans = ans % MOD;
            q++;
        }
    }
    ans += (x[n-1] - x[n-2]);
    ans = ans % MOD;
    for(lld i=2; i<=n-1; i++){
        ans *= i;
        ans = ans % MOD;
    }
    printf("%lld", ans);

    return 0;
}