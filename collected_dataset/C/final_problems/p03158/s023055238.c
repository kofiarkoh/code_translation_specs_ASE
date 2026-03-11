#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))
#define INF ((1LL<<62)-(1LL<<31))
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))
#define bit(n,m)(((n)>>(m))&1)
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}
ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
//#define MOD 998244353
#define MOD 1000000007
#define invp(a,p)pom(a,p-2,p)

ll n,q;
ll a[100010];
ll cum[100010];
ll cumo[100010];//cumo[i]=[0,i)の偶数indexの和

int bs(int x){
	//a[i]>xなる最小のi
	if(a[0]>x)return 0;
	ll l=0,r=n;//lはng,rはok
	while(r-l>1){
		int m=(l+r)/2;
		if(a[m]>x)r=m;
		else l=m;
	}
	return r;
}

int main(){
	scanf("%lld%lld",&n,&q);
	rep(i,0,n)scanf("%lld",a+i);
	rep(i,0,n)cum[i+1]=cum[i]+a[i];
	rep(i,0,n)cumo[i+1]=cumo[i]+(i%2==0?a[i]:0);
	rep(i,0,q){
		ll x;
		scanf("%lld",&x);
		//xがどこにあるかにぶたん
		ll idx=bs(x);
		//相手との衝突位置をにぶたん
		ll l=idx,r=n;//lはありえる、rはありえない
		while(r-l>1){
			int m=(l+r)/2;
			ll temp=llabs(a[m-1]-x);
			int iii=bs(x-temp-1);
			if(m-iii<=n-m)l=m;
			else r=m;
		}
//		printf("%lld ",l);
		//のこりは累積和でえい
		int iii=l-(n-l);
		ll ans=cum[l]-cum[iii];
		//のこりiii個を太郎から取る
		if(iii%2==0)ans+=cumo[iii];
		else ans+=cum[iii]-cumo[iii];
		printf("%lld\n",cum[n]-ans);
	}
}
