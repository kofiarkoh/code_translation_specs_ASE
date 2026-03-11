#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define INF ((1LL<<62)-(1LL<<31))
#define min(p,q)((p)<(q)?(p):(q))
#define chmin(p,q)(p=min(p,q))
#define bit(n,m)((n>>m)&1)

//辺の情報を個別に持つタイプ
typedef struct edge{ll s,g,c;}E;
typedef struct graph{
	int vcnt,ecnt;
	E  e[2010];//適宜変える
	int id[20];//適宜変える
}G;
int deg[20];
int esort(const void*a,const void*b){
	E*p=(E*)a,*q=(E*)b;
	if((*p).s<(*q).s)return -1;
	if((*p).s>(*q).s)return  1;
	if((*p).g<(*q).g)return -1;
	return 1;
}

G g;
void readgraph(){
	//適宜変える
	ll n,m;
	scanf("%lld%lld",&n,&m);
	rep(i,0,m){
		ll x,y,c;
		scanf("%lld%lld%lld",&x,&y,&c);
		deg[x]++;
		deg[y]++;
		g.e[2*i].s=x;
		g.e[2*i].g=y;
		g.e[2*i].c=c;
		g.e[2*i+1].s=y;
		g.e[2*i+1].g=x;
		g.e[2*i+1].c=c;
	}
	g.vcnt=n;
	g.ecnt=2*m;
	qsort(g.e,g.ecnt,sizeof(E),esort);

	int p=0;
	rep(i,0,g.vcnt){
		while(p<g.ecnt&&g.e[p].s<i)p++;
		g.id[i]=p;
	}
	g.id[g.vcnt]=g.ecnt;//番兵
}


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


int dp[1<<15];
int main(){
	readgraph();
	for(int i=0;i<g.vcnt;i++)for(int j=0;j<g.vcnt;j++)cost[i][j]=INF;
	for(int i=0;i<g.ecnt;i++)chmin(cost[g.e[i].s][g.e[i].g],g.e[i].c);
	waasharuhuroido(g.vcnt);
	
	for(int i=1;i<1<<g.vcnt;i++){
		int b=__builtin_popcount(i);
		if(b%2)continue;
		dp[i]=3e6;
		int j=0;
		while(!bit(i,j))j++;
		for(int k=j+1;k<g.vcnt;k++)if(bit(i,k)){
			chmin(dp[i],dp[i^(1<<j)^(1<<k)]+dddd[j][k]);
		}
	}

	int tans=0;
	for(int i=0;i<g.ecnt;i++)tans+=g.e[i].c;
	tans/=2;
	int flag=0;
	for(int i=0;i<g.vcnt;i++)flag|=(deg[i]%2)<<i;
	int ans=3e6;
	if(__builtin_popcount(flag)%2){
		for(int k=0;k<g.vcnt;k++)if(!bit(flag,k))chmin(ans,tans+dp[flag|(1<<k)]);
	}else ans=tans+dp[flag];
	
	printf("%d\n",ans);
}
