#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define INF ((1LL<<62)-(1LL<<31))

//*
//プラキュー（二分ヒープ）（最低限バージョン）
//int PQhikaku(int i,int j);//jの方が優先度が高いならtrueを返す
int PQhikaku(ll*heap,int i,int j){return heap[j]>heap[i];}
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

int idx[200010];
int t[200010],k[200010],x[200010];
ll ans[200010];
ll heap[200010];

int c(const void*a,const void*b){
	if(k[*(int*)a]<k[*(int*)b])return -1;
	if(k[*(int*)a]>k[*(int*)b])return 1;
	if(*(int*)a<*(int*)b)return -1;
	return 1;
}

int main(){
	int n,q;
	scanf("%d%d",&n,&q);
	for(int i=0;i<q;i++){
		scanf("%d%d",t+i,k+i);
		if(t[i]==0)scanf("%d",x+i);
		idx[i]=i;
		ans[i]=INF;
	}
	qsort(idx,q,sizeof(int),c);
	for(int ii=0;ii<q;ii++){
		int i=idx[ii];
		if(t[i]==0)PQpush(heap,x[i]);
		else if(t[i]==1&&heap[0])ans[i]=heap[1];
		else if(t[i]==2&&heap[0])PQpop(heap);
		
		if(k[i]!=k[idx[ii+1]])heap[0]=0;
	}
	for(int i=0;i<q;i++)if(ans[i]!=INF)printf("%lld\n",ans[i]);
}
