#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define INF ((1LL<<61)-(1LL<<31))
#define min(p,q)((p)<(q)?(p):(q))

//ワーシャルフロイド
//頂点数（と隣接行列）を引いて全点間最短距離を返す
//O(V^3)
//*
ll cost[110][110];
ll dddd[110][110];
void waasharuhuroido(ll n){
	rep(i,0,n)rep(j,0,n)dddd[i][j]=cost[i][j];//辺が無いところはINFにする前処理が必要
	rep(k,0,n)rep(i,0,n)rep(j,0,n)dddd[i][j]=min(dddd[i][j],dddd[i][k]+dddd[k][j]);
}
//*/

int main(){
	int n,m,s,g1,g2;
	while(scanf("%d%d%d%d%d",&n,&m,&s,&g1,&g2),n){
		s--,g1--,g2--;
		rep(i,0,n)rep(j,0,n)cost[i][j]=(i==j?0:INF);
		rep(_,0,m){
			int x,y,z;
			scanf("%d%d%d",&x,&y,&z);
			x--,y--;
			cost[x][y]=z;
		}
		waasharuhuroido(n);
		ll ans=INF;
		rep(i,0,n)ans=min(ans,dddd[s][i]+dddd[i][g1]+dddd[i][g2]);
		printf("%lld\n",ans);
	}
}
