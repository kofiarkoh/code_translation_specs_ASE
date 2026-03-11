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



//*
//プラキュー（二分ヒープ）（最低限バージョン）
//int PQhikaku(int i,int j);//jの方が優先度が高いならtrueを返す
int PQhikaku(ll*heap,int i,int j){return heap[i]>heap[j];}
void heap_utod(ll*heap,int n){
	if(2*n>heap[0])return;
	int rflag=(2*n+1<=heap[0])&&(PQhikaku(heap,2*n,2*n+1));
	if(PQhikaku(heap,n,2*n+rflag)){
		ll temp=heap[2*n+rflag];
		heap[2*n+rflag]=heap[n];
		heap[n]=temp;
		heap_utod(heap,2*n+rflag);
	}
}
void heap_dtou(ll*heap,int n){
	if(n==1||PQhikaku(heap,n,n/2))return;
	ll temp=heap[n];
	heap[n]=heap[n/2];
	heap[n/2]=temp;
	heap_dtou(heap,n/2);
}
ll PQpop(ll*heap){
	ll rr=heap[1];
	heap[1]=heap[heap[0]--];
	heap_utod(heap,1);
	return rr;
}
void PQpush(ll*heap,ll n){
	heap[++heap[0]]=n;
	heap_dtou(heap,heap[0]);
}
//*/


//セグ木ここから
//*
//↓ここを変える
typedef struct atai{ll a,i;}atai;
atai xx(atai x,atai y){
	atai r;
	r.a=max(x.a,y.a);
	r.i=x.a>y.a?x.i:y.i;
	return r;
}
atai aid={0};
//↑ここを変える

int segNUM;
atai *segN,*seg;
void seguse(int n){
	segN=(atai*)calloc(2*n,sizeof(atai));
	segNUM=n;
	seg=segN+segNUM;
	rep(i,1,2*segNUM)segN[i]=aid;
}
//seg[]に値を与えてから初期化
void seginit(){for(int node=segNUM-1;node;node--)segN[node]=xx(segN[node*2],segN[node*2+1]);}
void segupdate(int node,atai x){
	//seg[node]をxに更新
	node+=segNUM;
	segN[node]=x;
	while(node/=2)segN[node]=xx(segN[node*2],segN[node*2+1]);
}

atai segcalcsub(int l,int r,int k,int cl,int cr){
	if(l>=r)return aid;
	//完全に含むとき
	if(l<=cl&&cr<=r)return segN[k];

	int cm=(cl+cr)/2;
	//左側だけ
	if(r<=cm)return segcalcsub(l,r,2*k  ,cl,cm);
	//右側だけ
	if(cm<=l)return segcalcsub(l,r,2*k+1,cm,cr);
	//両方
	return xx(segcalcsub(l,r,2*k,cl,cm),segcalcsub(l,r,2*k+1,cm,cr));
}
atai segcalc(int l,int r){return segcalcsub(l,r,1,0,segNUM);}
//セグ木ここまで
//*/

int a[100010];
ll h[100010];
int ans[100010];
int main(){
	int n;
	scanf("%d",&n);
	seguse(1<<17);
	rep(i,1,n+1){
		scanf("%d",a+i);
		seg[a[i]].a++;
		seg[i].i=i;
		PQpush(h,i);
	}
	
	seginit();
	if(segcalc(1,n+1).a==n-1){
		puts("-1");
		return 0;
	}
	
	rep(i,1,n+1){
		ll back=-1;
		ll v=PQpop(h);
		if(i!=1&&v==a[ans[i-2]]){
			back=v;
			v=PQpop(h);
		}
		
		atai x={seg[a[v]].a-1,a[v]};
		segupdate(a[v],x);
//		rep(i,1,n+1)printf("%d ",seg[i].a);puts("");
//		printf("%d %d %d %d\n",i,v,segcalc(1,v).a,segcalc(v+1,n+1).a);
		atai hoge=segcalc(1,n+1);
		if(hoge.a==n-1-i&&hoge.i==a[v]){
			ll vv=PQpop(h);
			atai y={seg[a[vv]].a-1,a[vv]};
			segupdate(a[vv],y);
			PQpush(h,v);
			x.a++;
			segupdate(a[v],x);
			v=vv;
		}
		ans[i-1]=v;
		if(back!=-1)PQpush(h,back);
	}
	rep(i,0,n)printf("%d ",ans[i]);
}