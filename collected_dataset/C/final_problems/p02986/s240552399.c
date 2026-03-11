#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define bit(n,m)(((n)>>(m))&1)



//辺の情報を個別に持つタイプ
typedef struct edge{ll s,g,color,cost;}E;
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
ll q;
void readgraph(){
	//適宜変える
	ll n;
	scanf("%lld%lld",&n,&q);
	rep(i,0,n-1){
		ll x,y,col,cos;
		scanf("%lld%lld%lld%lld",&x,&y,&col,&cos);
		x--,y--;
		g.e[2*i].s=x;
		g.e[2*i].g=y;
		g.e[2*i].color=col;
		g.e[2*i].cost=cos;
		g.e[2*i+1].s=y;
		g.e[2*i+1].g=x;
		g.e[2*i+1].color=col;
		g.e[2*i+1].cost=cos;
	}
	g.vcnt=n;
	g.ecnt=2*n-2;
	qsort(g.e,g.ecnt,sizeof(E),esort);

	int p=0;
	rep(i,0,g.vcnt){
		while(p<g.ecnt&&g.e[p].s<i)p++;
		g.id[i]=p;
	}
	g.id[g.vcnt]=g.ecnt;//番兵
}

#define VCNT 100010
int dep[VCNT];
int oya[20][VCNT];
#undef VCNT
ll lcainitdfs(int v){
	rep(i,g.id[v],g.id[v+1]){
		if(!dep[g.e[i].g]){
			oya[0][g.e[i].g]=v;
			dep[g.e[i].g]=dep[v]+1;
			lcainitdfs(g.e[i].g);
		}
	}
}
void lcainit(int root){
	dep[root]=1;
	lcainitdfs(root);
	rep(k,1,20)rep(v,0,g.vcnt)oya[k][v]=oya[k-1][oya[k-1][v]];
}
int lca(int u,int v){
	int du=dep[u];
	int dv=dep[v];
	if(du<dv){
		int t;
		t=du;du=dv;dv=t;
		t= u; u= v; v=t;
	}
	rep(k,0,20)if(bit(du-dv,k))u=oya[k][u];

	if(u==v)return u;
	for(int k=19;k>=0;k--)if(oya[k][u]!=oya[k][v]){
		u=oya[k][u];
		v=oya[k][v];
	}
	return oya[0][u];
}

int u[100010];
int v[100010];
int l[100010];
int color[100010];
int cost[100010];
int pos[100010];
int crrpos[100010];

int queryidx[300010];
ll ans[100010];
ll ccnt[100010];
ll ccost[100010];
ll totalcost;

void dfs(int v,int pre){
	rep(idx,pos[v],pos[v+1]){
		int i=queryidx[idx];
		int flag=1;
		if(i<0){
			i=-i;
			flag=-2;
		}
		i--;
		ll val=totalcost-ccost[color[i]]+ccnt[color[i]]*cost[i];
		ans[i]+=val*flag;
	}
	rep(i,g.id[v],g.id[v+1]){
		if(g.e[i].g!=pre){
			ccnt [g.e[i].color]++;
			ccost[g.e[i].color]+=g.e[i].cost;
			totalcost+=g.e[i].cost;
			dfs(g.e[i].g,v);
			totalcost-=g.e[i].cost;
			ccost[g.e[i].color]-=g.e[i].cost;
			ccnt [g.e[i].color]--;			
		}
	}
}

int main(){
	readgraph();
	lcainit(0);
	rep(i,0,q){
		scanf("%d%d%d%d",color+i,cost+i,u+i,v+i);
		u[i]--,v[i]--;
	}
	rep(i,0,q){
		l[i]=lca(u[i],v[i]);
		pos[u[i]+1]++;
		pos[v[i]+1]++;
		pos[l[i]+1]++;
	}
	rep(i,1,g.vcnt+3)crrpos[i]=pos[i]+=pos[i-1];
	rep(i,0,q){
		queryidx[crrpos[u[i]]++]=i+1;
		queryidx[crrpos[v[i]]++]=i+1;
		queryidx[crrpos[l[i]]++]=-i-1;
	}
	dfs(0,-1);
	rep(i,0,q)printf("%lld\n",ans[i]);
}