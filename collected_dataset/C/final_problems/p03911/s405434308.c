#include <stdio.h>
#include <stdlib.h>
#define FOR(i,a,n) for(i=a;i<n;i++)
//int *v,*e;
int *par;
int *rank;
void init_union(int size){
	int i;
	par = (int*)calloc(size,sizeof(int));
	rank = (int*)calloc(size,sizeof(int));
	//v = (int*)calloc(size,sizeof(int));
	//e = (int*)calloc(size,sizeof(int));
	FOR(i,0,size) par[i]=i;
	return;
}

int root(int a){
 	if(par[a]==a) return a;
 	else{
 		par[a]=root(par[a]);
 		return par[a];
 	}
}
 
int is_same(int a,int b){
 	return root(a)==root(b);
}
 
void unite(int a,int b){
	a=root(a);
	b=root(b);
	if(a==b){
		//e[a]++;
		return;
	}
	if(rank[a]>rank[b]){
		par[b]=a;
		//v[a]++,e[a]++;
	}
	else{
		par[a]=b;
		//v[b]++,e[b]++;
		if(rank[a]==rank[b]) rank[b]++;
	}
	return;
}
int main(void)
{
	int n,m,k,l,i;
	scanf("%d%d",&n,&m);
	init_union(n+m);
	FOR(i,0,n){
		scanf("%d",&k);
		while(k-->0){
			scanf("%d",&l);
			unite(i,n+l-1);
		}
	}
	FOR(i,1,n){
		if(!is_same(0,i)){
			puts("NO");
			return 0;
		}
	}
	puts("YES");
	return 0;
}
