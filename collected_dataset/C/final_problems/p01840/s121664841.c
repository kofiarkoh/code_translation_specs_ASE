#include <stdio.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)

ll a[20010];
int main(){
	ll n,m,t;
	scanf("%lld%lld%lld",&n,&m,&t);
	rep(_,0,n){
		ll x;
		scanf("%lld",&x);
		rep(i,x-m,x+m)a[i]=1;
	}
	ll ans=0;
	rep(i,0,t)ans+=!a[i];
	printf("%lld\n",ans);
}
