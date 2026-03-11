#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))

ll a[110];
int main(){
	ll n;
	scanf("%lld",&n);
	rep(i,0,n){
		ll t;
		scanf("%lld",&t);
		a[t]++;
	}
	ll ans=0;
	rep(i,0,105)ans+=!!a[i];
	printf("%lld",ans);
}