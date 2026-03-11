#include<stdio.h>
#define MAX_N 150
#define INF 10e8
typedef struct {
	int a;
	int b;
	int w;
} hen;

int par[MAX_N];
int rank[MAX_N];
int n,m;
hen dis[20000];

void init(int n){
	int i;
	for(i=0;i<n;i++){
		par[i] = i;
		rank[i] = 0;
	}
}
int find(int x){
	if(par[x]==x){
		return x;
	}else {
		return par[x] = find(par[x]);
	}
}
void unite(int x,int y){
	x =find(x);
	y =find(y);
	if(x==y){
		return ;
	}
	if(rank[x] < rank[y]){
		par[x] = y;
	}else {
		par[y] = x;
		if(rank[x] == rank[y])rank[x]++;
	}
}
int same(int x,int y){
	return find(x)==find(y);
}
int  judge(hen *a,hen *b){
	return (a->w)-(b->w);
}

int Kruskal(int start){
	int eda=0;
	int i,j,k;
	int max=0;
	int min = dis[start].w;
	init(110);
	for(i=start;i<m;i++){
		hen h = dis[i];
		if(!same(h.a,h.b)){
			eda++;
			unite(h.a,h.b);
			max = h.w;
		}
	}
	
	for(i=1;i<n+1;i++){
	//	printf("%d ",find(i));
		if(!same(1,i)){
			return -1;
		}
	}
	if(eda==n-1){
		return max-min;
	}else {
		return INF;
	}
}
int main(){
	int i,j,k;
	int ans=INF;
	int buf=0;
	while(scanf("%d %d",&n,&m)){
		if(n==0&&m==0){
			break;
		}
		for(i=0;i<m;i++){
			scanf("%d %d %d",&dis[i].a,&dis[i].b,&dis[i].w);
		}
		ans=10e8;
		qsort(dis,m,sizeof(hen),judge);
		for(i=0;i<m;i++){
			buf = Kruskal(i);
			if(buf!=-1&&buf<ans){
				ans =buf;
			}
		}
		if(ans==INF){
			printf("%d\n",-1);
		}else {
			printf("%d\n",ans);
		}
	}
	return 0;
}