#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define ll long long
#define rep(i,l,r)for(int i=(l);i<(r);i++)
#define INF ((1LL<<62)-(1LL<<31))
#define MOD 1000000007

//辺の情報を個別に持つタイプ
typedef struct edge{ll s,g,c;}E;
typedef struct graph{
	int vcnt,ecnt;
	E  e[2000010];//適宜変える(ecnt)
	int id[250010];//適宜変える(vcnt)
}G;
G g;

int esort(const void*a,const void*b){
	E*p=(E*)a,*q=(E*)b;
	if((*p).s<(*q).s)return -1;
	if((*p).s>(*q).s)return  1;
	if((*p).g<(*q).g)return -1;
	return 1;
}

ll n,m;
int d4[]={0,1,0,-1,0};
#define in(i,j)(0<=i&&i<n&&0<=j&&j<m)
void readgraph(){
	//適宜変える
	scanf("%lld%lld",&n,&m);
	ll e=0;
	rep(i,0,n)rep(j,0,m){
		rep(k,0,4){
			int ii=i+d4[k];
			int jj=j+d4[k+1];
			if(in(ii,jj)){
				g.e[e].s= i*m+ j;
				g.e[e].g=ii*m+jj;
				g.e[e].c=1;
				e++;
			}
		}
		g.e[e].s= i*m+ j;
		g.e[e].g= 0*m+ j;
		g.e[e].c=1;
		e++;
		g.e[e].s= i*m+ j;
		g.e[e].g= i*m+ 0;
		g.e[e].c=1;
		e++;
		g.e[e].s= i*m+ j;
		g.e[e].g=(n-1)*m+j;
		g.e[e].c=1;
		e++;
		g.e[e].s= i*m+ j;
		g.e[e].g= i*m+(m-1);
		g.e[e].c=1;
		e++;
	}
	g.vcnt=n*m;
	g.ecnt=e;
	
	qsort(g.e,g.ecnt,sizeof(E),esort);
//	qsort(g.e,g.ecnt,sizeof(E),csort);
//	makeinvedge();

	int p=0;
	rep(i,0,g.vcnt){
		while(p<g.ecnt&&g.e[p].s<i)p++;
		g.id[i]=p;
	}
	g.id[g.vcnt]=g.ecnt;//番兵
}


//プラキュー（二分ヒープ）（優先度変更ありバージョン）
ll heapN,heap[1<<20],heapinv[1<<20];
int PQhikaku(int i,int j);//jの方が優先度が高いならtrueを返す
void PQchange(int n);
void heap_utod(int n){
	if(2*n>heapN)return;
	int rflag=(2*n+1<=heapN)&&(PQhikaku(2*n,2*n+1));
	if(PQhikaku(n,2*n+rflag)){
		ll temp=heap[2*n+rflag];
		heap[2*n+rflag]=heap[n];
		heap[n]=temp;
		heapinv[heap[n]]=n;
		heapinv[heap[2*n+rflag]]=2*n+rflag;	
		heap_utod(2*n+rflag);
	}
}
void heap_dtou(int n){
	if(n==1||PQhikaku(n,n/2))return;
	ll temp=heap[n];
	heap[n]=heap[n/2];
	heap[n/2]=temp;
	heapinv[heap[n]]=n;
	heapinv[heap[n/2]]=n/2;
	heap_dtou(n/2);
}
ll PQpop(){
	ll rr=heap[1];
	heapinv[heap[1]]=0;
	heap[1]=heap[heapN--];
	heapinv[heap[1]]=1;
	heap_utod(1);
	return rr;
}
void PQpush(ll n){
	heap[++heapN]=n;
	heapinv[heap[heapN]]=heapN;
	heap_dtou(heapN);
}

//早い方のダイクストラ
//グラフと始点を引いて各点への最短距離・最短経路を返す
//プラキューが必要
//O((E+V)logV)
ll daikusutorappp[1<<20];
ll daikusutoracnt[1<<20];
void daikusutora2(ll s){
	rep(i,0,g.vcnt){
		daikusutorappp[i]=i==s?0:INF;
		daikusutoracnt[i]=i==s?1:0;
	}
	rep(i,0,g.vcnt)PQpush(i);
	while(heapN){
		ll mv=PQpop();
		for(ll t=g.id[mv];t<g.ecnt&&g.e[t].s==mv;t++){
			if(daikusutorappp[g.e[t].g]==daikusutorappp[mv]+g.e[t].c){
				daikusutoracnt[g.e[t].g]+=daikusutoracnt[mv];
				daikusutoracnt[g.e[t].g]%=MOD;
			}else if(daikusutorappp[g.e[t].g]>daikusutorappp[mv]+g.e[t].c){
				daikusutorappp[g.e[t].g]=daikusutorappp[mv]+g.e[t].c;
				daikusutoracnt[g.e[t].g]=daikusutoracnt[mv];
				PQchange(g.e[t].g);
			}
		}
	}
}
int PQhikaku(int i,int j){return daikusutorappp[heap[i]]>daikusutorappp[heap[j]];}
void PQchange(int n){if(heapinv[n])heap_dtou(heapinv[n]);}

int main(){
	readgraph();
	int a,b,c,d;
	scanf("%d%d%d%d",&a,&b,&c,&d);
	daikusutora2(a*m+b);
	printf("%d %d\n",daikusutorappp[c*m+d],daikusutoracnt[c*m+d]);
}
