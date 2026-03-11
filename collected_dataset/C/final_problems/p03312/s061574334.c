#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))
#define INF ((1LL<<62)-(1LL<<31))
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))
#define bit(n,m)((n>>m)&1)
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}

ll n,ans;
ll a[200010];
ll b[200010];

ll bs(ll i,ll s){
	//[i,r)<sなる最大のrを返す
	ll l=i,r=n;
	while(r-l>1){
		ll m=(l+r)/2;
		if(b[m]-b[i]<s)l=m;
		else r=m;
	}
	return l;
}

//ダメそう
int main(){
	scanf("%lld",&n);
	rep(i,0,n)scanf("%lld",a+i);
	rep(i,0,n)b[i+1]=b[i]+a[i];
	
	ll ans=INF;
	rep(i,1,n-2){
		//[0,i)を最初にとって、残りを3等分
		ll left=bs(i,(b[n]-b[i])/3);
		rep(l,left-5,left+6)if(i<l&&l<n-1){
			ll right=bs(l,(b[n]-b[l])/2);
			rep(r,right-5,right+6)if(l<r&&r<n){
//				printf("%d %d %d\n",i,l,r);
				ll x1=b[i]-b[0];
				ll x2=b[l]-b[i];
				ll x3=b[r]-b[l];
				ll x4=b[n]-b[r];
				ll temp=max(x1,max(x2,max(x3,x4)))-min(x1,min(x2,min(x3,x4)));
				ans=min(ans,temp);
			}
		}
	}
	printf("%lld\n",ans);
	return 0;
}