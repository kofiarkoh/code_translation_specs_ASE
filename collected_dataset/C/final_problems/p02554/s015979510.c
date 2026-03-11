#include <stdio.h>
typedef long long ll;
#define rep(i,n) for(int i=0; i<(int)(n); i++)
/////calc (mod MOD)
const ll MOD = 1000000007;
/////nCk (mod MOD) BEGIN/////
//faccal -> n!, k!^(MOD-2)()
//comb -> nCk (mod MOD)
ll mlpow(ll x, ll p) { //(x^p) % MOD
    ll tmp = 1;
    if(p == 0) return 1;
    while(p != 0) {
        if(p & 1) tmp = tmp*x % MOD;
        x = x*x % MOD;
        p = p >> 1;
    }
    return tmp;
}

#define facMAX 2000000
ll fac[facMAX];     //n! (mod MOD)
ll facM2[facMAX];   //(k!)^(MOD-2) (mod MOD)
void faccal(ll up) {
    fac[0] = 1;
    facM2[0] = 1;
    for(int i = 0; i < up + 1; i++){
        fac[i+1] = fac[i] * (i+1) % MOD;
        facM2[i+1] = facM2[i] * mlpow(i+1, MOD-2) % MOD;
    }
}

//nCk = n!*(k!)^(M-2)*((n-k)!)^(M-2)
ll comb(ll n, ll k) {
    if(n == 0 && k == 0) return 1;
    if(n < k || n < 0) return 0;
    ll tmp = (facM2[n-k] * facM2[k]) % MOD;
    return tmp * fac[n] % MOD;
}

int main(){
    ll n; scanf("%lld", &n);
    ll ans = 0;
    if(n > 1){
        ans = mlpow(10, n) + mlpow(8, n);
        ans %= MOD;
        ans += (2*MOD-mlpow(9, n)*2)%MOD;
        ans %= MOD;
    }

    printf("%lld\n", ans);
}
