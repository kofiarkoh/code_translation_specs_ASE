#include <stdio.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define max(p,q)((p)>(q)?(p):(q))

ll a[60];
int main(){
	ll n,l,r;
	while(scanf("%lld%lld%lld",&n,&l,&r),n){
		ll ans=0;
		rep(i,0,n)scanf("%lld",a+i);
		rep(x,l,r+1){
			int flag=-1;
			rep(i,0,n)if(x%a[i]==0){
				flag=(i+1)%2;
				break;
			}
			if(flag==-1)flag=(n+1)%2;
			ans+=flag;
		}
		printf("%lld\n",ans);
	}
}

