#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
#define INF ((1LL<<62)-(1LL<<31))

//辺の情報を個別に持つタイプ
typedef struct edge{int s,g,c;}E;
typedef struct graph{
	int vcnt,ecnt;
	E  e[12000010];//適宜変える(ecnt)
	int id[1000010];//適宜変える(vcnt)
}G;
G g;

int esort(const void*a,const void*b){
	E*p=(E*)a,*q=(E*)b;
	if((*p).s<(*q).s)return -1;
	if((*p).s>(*q).s)return  1;
	if((*p).g<(*q).g)return -1;
	return 1;
}

#define in(ii,jj)(0<=ii&&ii<n&&0<=jj&&jj<m)
char s[1010][1010];
int  d4[]={0,1,0,-1,0};
int dd4[]={1,1,-1,-1,1};
int start,goal;
void readgraph(){
	//適宜変える
	ll n,m,a,b;
	scanf("%lld%lld%lld%lld",&n,&m,&a,&b);
	rep(i,0,n)scanf(" %s",s[i]);
	ll ecnt=0;
	rep(i,0,n)rep(j,0,m){
		if(s[i][j]=='s')start=i*m+j;
		if(s[i][j]=='g')goal=i*m+j;
		rep(k,0,4){
			int ii=i+d4[k];
			int jj=j+d4[k+1];
			if(in(ii,jj)&&s[ii][jj]!='#'){
				g.e[ecnt].s= i*m+ j;
				g.e[ecnt].g=ii*m+jj;
				g.e[ecnt].c=a;
				ecnt++;
			}
		}
		int flag=s[i][j]!='*';
		rep(k,0,4){
			int ii=i+d4[k];
			int jj=j+d4[k+1];
			flag&=!in(ii,jj)||s[ii][jj]!='*';
			ii=i+dd4[k];
			jj=j+dd4[k+1];
			flag&=!in(ii,jj)||s[ii][jj]!='*';
		}
		if(flag){
			rep(k,0,4){
				int ii=i+d4[k];
				int jj=j+d4[k+1];
				if(in(ii,jj)){
					g.e[ecnt].s= i*m+ j;
					g.e[ecnt].g=ii*m+jj;
					g.e[ecnt].c=b+a;
					ecnt++;
				}
			}
			rep(k,0,4){
				int ii=i+dd4[k];
				int jj=j+dd4[k+1];
				if(in(ii,jj)){
					g.e[ecnt].s= i*m+ j;
					g.e[ecnt].g=ii*m+jj;
					g.e[ecnt].c=b+a+a;
					ecnt++;
				}
			}
		}
	}
	g.vcnt=n*m;
	g.ecnt=ecnt;
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
ll daikusutorappp[1<<21];
void daikusutora2(ll s){
	rep(i,0,g.vcnt)daikusutorappp[i]=i==s?0:INF;
	rep(i,0,g.vcnt)PQpush(i);
	while(heapN){
		ll mv=PQpop();
		for(ll t=g.id[mv];t<g.ecnt&&g.e[t].s==mv;t++){
			if(daikusutorappp[g.e[t].g]>daikusutorappp[mv]+g.e[t].c){
				daikusutorappp[g.e[t].g]=daikusutorappp[mv]+g.e[t].c;
				//daikusutorappp[n+g.e[t].g]=mv;//最短経路がいる場合はこれ
				PQchange(g.e[t].g);
			}
		}
	}
}
int PQhikaku(int i,int j){return daikusutorappp[heap[i]]>daikusutorappp[heap[j]];}
void PQchange(int n){if(heapinv[n])heap_dtou(heapinv[n]);}

int main(){
	readgraph();
	daikusutora2(start);
	printf(daikusutorappp[goal]==INF?"INF\n":"%lld\n",daikusutorappp[goal]);
}

