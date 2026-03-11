#include <stdio.h>
#include <string.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define repp(i,l,r,k)for(ll i=(l);i<(r);i+=(k))
#define INF ((1LL<<62)-(1LL<<31))
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)<(q)?(p):(q))
int upll(const void*a, const void*b){return*(ll*)a<*(ll*)b?-1:*(ll*)a>*(ll*)b?1:0;}
int downll(const void*a, const void*b){return*(ll*)a<*(ll*)b?1:*(ll*)a>*(ll*)b?-1:0;}
void sortup(ll*a,int n){qsort(a,n,sizeof(ll),upll);}
void sortdown(ll*a,int n){qsort(a,n,sizeof(ll),downll);}

ll x[1500010];
ll y[1500010];
ll xx[1510],yy[1510];
int main(){
	ll n,m;
	while(scanf("%lld%lld",&n,&m),n){
		ll s=0,t;
		xx[0]=yy[0]=0;
		rep(i,0,n){
			scanf("%lld",&t);
			s+=t;
			xx[i+1]=s;
		}
		s=0;
		rep(i,0,m){
			scanf("%lld",&t);
			s+=t;
			yy[i+1]=s;
		}
		rep(i,0,1500010)x[i]=y[i]=0;
		rep(i,0,n+1)rep(j,i+1,n+1)x[xx[j]-xx[i]]++;
		rep(i,0,m+1)rep(j,i+1,m+1)y[yy[j]-yy[i]]++;
		ll ans=0;
		rep(i,1,1500010)ans+=x[i]*y[i];
		printf("%lld\n",ans);
	}
	return 0;
}
