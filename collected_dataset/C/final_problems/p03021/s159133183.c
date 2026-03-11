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

char s[2010];
//辺の情報を個別に持つタイプ
typedef struct edge{ll s,g,c;}E;
typedef struct graph{
	int vcnt,ecnt;
	E  e[200010];//適宜変える
	int id[100010];//適宜変える
}G;

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
	scanf("%lld %s",&n,s);
	rep(i,0,n-1){
		ll x,y,c;
		scanf("%lld%lld",&x,&y);
		x--,y--;
		g.e[2*i].s=x;
		g.e[2*i].g=y;
		g.e[2*i+1].s=y;
		g.e[2*i+1].g=x;
	}
	g.vcnt=n;
	g.ecnt=2*(n-1);
	qsort(g.e,g.ecnt,sizeof(E),esort);

	int p=0;
	rep(i,0,g.vcnt){
		while(p<g.ecnt&&g.e[p].s<i)p++;
		g.id[i]=p;
	}
	g.id[g.vcnt]=g.ecnt;//番兵
}



//dfs
ll sum;
ll dfs(int v,int pre,int d){
	if(s[v]=='1')sum+=d;
	rep(i,g.id[v],g.id[v+1]){
		if(g.e[i].g!=pre){
			dfs(g.e[i].g,v,d+1);
		}
	}
}

ll f(ll v){
	ll S=0,M=0;
	rep(i,g.id[v],g.id[v+1]){
		sum=0;
		dfs(g.e[i].g,v,1);
		S+=sum;
		M=max(M,sum);
	}	
	if(S%2==0&&M<=S/2)return S/2;
	return INF;
}

int main(){
	readgraph();
	{
		ll cnt=0;
		rep(i,0,g.vcnt)if(s[i]=='1')cnt++;
		if(cnt==1){
			puts("0");
			return 0;
		}
	}
	
	ll ans=INF;
	rep(i,0,g.vcnt){
		ll temp=f(i);
		ans=min(ans,temp);
	}
	printf("%lld",ans==INF?-1:ans);
}