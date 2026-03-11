#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
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

ll L;
//ワーシャルフロイド
//頂点数（と隣接行列）を引いて全点間最短距離を返す
//O(V^3)
ll cost[310][310];
ll dddd[310][310];
ll cccc[310][310];
void waasharuhuroido(ll n){
	rep(i,0,n)rep(j,0,n)if(i!=j){
		if(cost[i][j]&&cost[i][j]<=L){
			dddd[i][j]=cost[i][j];
		}else{
			dddd[i][j]=cccc[i][j]=INF;
		}
	}
	rep(_,0,3)rep(k,0,n)rep(i,0,n)rep(j,0,n){
		ll tempc=cccc[i][k]+cccc[k][j];
		ll tempd=dddd[i][k]+dddd[k][j];
		if(!(tempc==0&&tempd<=L)){
			tempc++;
			tempd=dddd[k][j];
		}
		
		if(cccc[i][j]>tempc||(cccc[i][j]==tempc&&dddd[i][j]>tempd)){
			cccc[i][j]=tempc;
			dddd[i][j]=tempd;
		}
//		rep(i,0,n){rep(j,0,n)printf("(%2d,%2d) ",cccc[i][j]==INF?99:cccc[i][j],dddd[i][j]==INF?99:dddd[i][j]);puts("");}puts("");
	}
}

int main(){
	ll n,m;
	scanf("%lld%lld%lld",&n,&m,&L);
	rep(i,0,m){
		ll x,y,z;
		scanf("%lld%lld%lld",&x,&y,&z);
		x--,y--;
		cost[x][y]=cost[y][x]=z;
	}
	waasharuhuroido(n);
	ll q;
	scanf("%lld",&q);
	while(q--){
		ll x,y;
		scanf("%lld%lld",&x,&y);
		x--,y--;
		ll ans=cccc[x][y];
		printf("%lld\n",ans==INF?-1:ans);
	}
}
