#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define min(p,q)((p)<(q)?(p):(q))

//辺の情報を個別に持つタイプ
typedef struct edge{ll s,g,c;}E;
typedef struct graph{
	int vcnt,ecnt;
	E  e[100010];//適宜変える
	int id[100010];//適宜変える
}G;

G g;
void readgraph(){
	//適宜変える
	ll n;
	scanf("%lld",&n);
	ll m=0;
	rep(x,0,n){
		int k;
		scanf("%d",&k);
		while(k--){
			ll y;
			scanf("%lld",&y);
			g.e[m].s=x;
			g.e[m].g=y;
			g.e[m].c=0;
			m++;
		}
	}
	g.vcnt=n;
	g.ecnt=m;

	int p=0;
	rep(i,0,g.vcnt){
		while(p<g.ecnt&&g.e[p].s<i)p++;
		g.id[i]=p;
	}
	g.id[g.vcnt]=g.ecnt;//番兵
}

//Euler Tour
#define VCNT 100000
int ein[VCNT+10],eout[VCNT+10];
int vin[VCNT+10],vout[VCNT+10];
int ETe[2*VCNT+10],ETv[2*VCNT+10];
int ETcnt=0;
#undef VCNT
void ETdfs(int v){
	vin[v]=ETcnt;
	ETv[ETcnt]=v;
	rep(i,g.id[v],g.id[v+1]){
		ein[i]=ETcnt;
		ETe[ETcnt]=i;
		ETcnt++;
		ETdfs(g.e[i].g);
		eout[i]=ETcnt;
		ETe[ETcnt]=i+g.ecnt;
		ETcnt++;
		ETv[ETcnt]=v;
	}
	vout[v]=ETcnt;
}
void makeET(int root){
	ETdfs(root);
	ETv[ETcnt]=root;
}

//深さ
#define VCNT 100010
int dep[VCNT];
#undef VCNT
void depsdfs(int v){
	rep(i,g.id[v],g.id[v+1]){
		dep[g.e[i].g]=dep[v]+1;
		depsdfs(g.e[i].g);
	}
}

//sparse table
int cum[20][200010];
void makeST(int n){
	rep(i,0,19)rep(j,0,n)cum[i+1][j]=min(cum[i][j],cum[i][min(j+(1<<i),n-1)]);
}

//ST+ETでLCA
int lcasub(int l,int r){
//	if(l==r)return INF;
	if(r<l){int t=l;l=r;r=t;}
	int cnt=r-l;
	int k=31-__builtin_clz(cnt);
	return min(cum[k][l],cum[k][r-(1<<k)]);
}
int lca(int u,int v){
	if(u==v)return dep[u];
	return lcasub(vin[u],vin[v]);
}

//トポソ
int idx[100010];
int c(const void*a,const void*b){
	if(dep[*(int*)a]>dep[*(int*)b])return -1;
	return 1;
}

ll crr[100010];
ll upd[100010];
int st[1010];
int updcnt;
void distdfs(int v){
	rep(i,g.id[v],g.id[v+1]){
		crr[g.e[i].g]=crr[v]+g.e[i].c;
		distdfs(g.e[i].g);
	}
}
void update(){
	rep(i,0,g.vcnt-1){
		int v=idx[i];
		int e=ETe[vout[v]]-g.ecnt;//vと親を結ぶ辺のindex
		g.e[e].c+=upd[v];
		upd[ETv[vout[v]+1]]+=upd[v];
		upd[v]=0;
	}
	distdfs(0);
}

int main(){
	readgraph();
	makeET(0);
	dep[0]=1;depsdfs(0);
	rep(i,0,g.vcnt)dep[i]--;
	rep(i,0,g.vcnt)idx[i]=i;
	qsort(idx,g.vcnt,sizeof(int),c);
	rep(i,0,g.ecnt*2)cum[0][i]=dep[ETv[i]];
	makeST(g.ecnt*2);

	int sqN=sqrt(g.vcnt);
	int q;
	scanf("%d",&q);
	while(q--){
		int t;
		scanf("%d",&t);
		if(t==0){
			int v,w;
			scanf("%d%d",&v,&w);
			if(upd[v])upd[v]+=w;
			else{
				upd[v]=w;
				st[updcnt++]=v;
				if(updcnt==sqN){
					//ならす
					update();
					updcnt=0;
				}
			}
		}else{
			int v;
			scanf("%d",&v);
			ll ans=crr[v];
			rep(i,0,updcnt)ans+=upd[st[i]]*lca(v,st[i]);
			printf("%lld\n",ans);
		}
//		rep(i,0,g.vcnt)printf("%d ",crr[i]);puts("");
//		rep(i,0,g.vcnt)printf("%d ",upd[i]);puts("");
//		puts("");
	}
}
