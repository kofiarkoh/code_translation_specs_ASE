#include <stdio.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define min(p,q)((p)<(q)?(p):(q))

ll a[2010];
ll s[2000010];
ll sum;
int main(){
	int n;
	scanf("%d",&n);
	rep(i,0,n){
		scanf("%lld",a+i);
		sum+=a[i];
	}
	//sum/2以上で最も小さいもの⇔sum-(sum/2以下で最も大きいもの)

	s[0]=1;
	ll csum=0;
	rep(i,0,n){
		csum=min(csum+a[i],sum/2);
		for(int j=csum;j>=a[i];j--)if(s[j-a[i]])s[j]=1;
	}
	for(ll ans=sum/2;;ans--)if(s[ans]){
		printf("%lld",sum-ans);
		return 0;
	}
}
