#include<stdio.h>
#include<stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)

//辺の情報を個別に持つタイプ
typedef struct edge{ll s,g,c;}E;
typedef struct graph{
	int vcnt,ecnt;
	E  e[2010];//適宜変える
	int id[1010];//適宜変える
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
	scanf("%lld%lld",&n,&m);
	if(n==0)exit(0);
	rep(i,0,m){
		ll x,y,c;
		scanf("%lld%lld",&x,&y);
		x--,y--;
		g.e[2*i].s=x;
		g.e[2*i].g=y;
		g.e[2*i+1].s=y;
		g.e[2*i+1].g=x;
	}
	g.vcnt=n;
	g.ecnt=2*m;
	qsort(g.e,g.ecnt,sizeof(E),esort);
//	qsort(g.e,g.ecnt,sizeof(E),csort);

	int p=0;
	rep(i,0,g.vcnt){
		while(p<g.ecnt&&g.e[p].s<i)p++;
		g.id[i]=p;
	}
	g.id[g.vcnt]=g.ecnt;//番兵
}



ll a[1010];
int flag;
//dfs
ll dfs(int v){
	rep(i,g.id[v],g.id[v+1]){
		ll nv=g.e[i].g;
		if(!a[nv]){
			a[nv]=a[v]^3;
			dfs(nv);
		}else{
			if(a[v]&a[nv])flag=0;
		}
	}
}

int main(){
	while(1){
		readgraph();
		rep(i,0,1005)a[i]=0;
		a[0]=1;
		flag=1;
		dfs(0);
		if(flag){
			int cnt=0;
			rep(i,0,g.vcnt)if(a[i]==1)cnt++;
			if(cnt%2==0){
				if((g.vcnt-cnt)%2==0){
					if(cnt>g.vcnt/2)cnt=g.vcnt-cnt;
					if(cnt*2==g.vcnt)printf("%d\n%d\n",1,cnt/2);
					else printf("%d\n%d\n%d\n",2,cnt/2,(g.vcnt-cnt)/2);
				}else{
					printf("%d\n%d\n",1,cnt/2);
				}
			}else{
				if((g.vcnt-cnt)%2==0){
					printf("%d\n%d\n",1,(g.vcnt-cnt)/2);
				}else{
					puts("0");
				}
			}
		}else{
			puts("0");
		}
	}
}
