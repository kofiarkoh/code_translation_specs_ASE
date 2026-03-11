#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdbool.h>
typedef long long ll;
#define rep(i,n) for(ll i=0; i<(ll)(n); i++)
#define max(l,r) ((l)>(r)?l:r)
#define min(l,r) ((l)<(r)?l:r)
#define swap(l,r) {ll tp=l;l=r;r=tp;}
const ll MOD = 1000000007;
ll lpow(ll x, ll p) {
    ll tmp = 1;
    if(p == 0) return 1;
    while(p != 0) {
        if(p & 1) tmp = tmp*x;
        x = x*x;
        p = p >> 1;
    }
    return tmp;
}

int main(){
    //input
    ll n, p;
    scanf("%lld%lld", &n, &p);
    
    //solve
    //素因数分解
    struct ret{
        ll val, cnt;
    };
    struct ret ret[101010];
    ll ntmp = p, rcnt = 0;
    for(ll i = 2; i * i <= ntmp; i++) {
        if(ntmp % i == 0){
            ret[rcnt].val = i;
            while(ntmp % i == 0) {
                ret[rcnt].cnt++;
                ntmp /= i;
            }
            rcnt++;
        }
    }
    if(ntmp != 1){
        ret[rcnt].val = ntmp;
        ret[rcnt].cnt = 1;
        rcnt++;
    }
    ll ans = 1;
    rep(i, rcnt){
        ans *= lpow(ret[i].val, ret[i].cnt/n);
    }

    //output
    printf("%lld", ans);
}