#include <stdio.h>
#include <stdlib.h>

#define ll long long
#define MD 1000000007
#define mod(n) ((n%MD+MD)%MD)
#define max(x, y) (x>y? x : y)
#define min(x, y) (x<y? x : y)
#define mul3(x, y, z) (mod(mod(x*y)*z)) 

ll perm(ll n, ll r){
    ll res = 1;
    for(ll i=0;i<r;i++){
        res = mod(res*(n-i));
    }
    return res;
}


ll inv(ll n){
    ll res = 1;
    ll a = MD-2;
    while(a){
        if(a&1) res = mod(res*n);
        n = mod(n*n);
        a >>= 1;
    }
    return res;
}

ll conb(ll n, ll r){
    r = min(r, n-r);
    ll a = perm(n, r);
    ll b = perm(r, r);
    ll res = mod(a*inv(b));
    return res;
}

ll modpow(ll x, ll y){
    ll res = 1;
    while(y>0){
        if(y&1) res = mod(res*x);
        x = mod(x*x);
        y >>= 1;
    }
    return res;
}

int main(){
    ll K;
    char S[1000000];
    scanf("%lld%s", &K, S);
    // end input

    ll s_len = 0;
    for(;s_len<(ll)1e6;s_len++){
        if(!S[s_len]) break;
    }
    
    ll ans = 0;
    for(ll i=K;i>=0;i--){
        ll t = mul3(conb(s_len+i-1, i), modpow(25, i), modpow(26, K-i));
        ans = mod(ans + t);
    }
    printf("%lld\n", ans);

    return 0;
}

