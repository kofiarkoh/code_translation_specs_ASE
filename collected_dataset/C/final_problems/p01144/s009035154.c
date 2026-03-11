#include <stdio.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))

ll a[20];
int main(){
	ll n,s;
	while(scanf("%lld%lld",&n,&s),n){
		rep(i,0,20)a[i]=0;
		rep(i,0,n){
			ll p,q;
			scanf("%lld%lld",&p,&q);
			a[q]+=p;
		}
		ll ans=0;
		for(int i=10;i>0;i--){
			ans+=max(0,a[i]-s)*i;
			s-=min(a[i],s);
		}
		printf("%lld\n",ans);
	}
}


