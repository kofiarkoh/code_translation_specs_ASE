#include<stdio.h>
#include<stdlib.h>

typedef struct UnionFind{
  int *parent;
  int n;
} UnionFind;

UnionFind* newUnionFind(int n){
  UnionFind *u=(UnionFind *)calloc(1,sizeof(UnionFind));
  u->parent=(int *)calloc(n,sizeof(int));
  u->n=n;
  for(int i=0;i<n;i++) u->parent[i]=-1;
  return u;
}

int root(UnionFind *u,int x){
  if(u->parent[x]<0) return x;
  return u->parent[x]=root(u,u->parent[x]);
}

int same(UnionFind *u,int x,int y){
  return root(u,x)==root(u,y);
}

int getSize(UnionFind *u,int x){
  return -u->parent[root(u,x)];
}

void unite(UnionFind *u,int x,int y){
  x=root(u,x);
  y=root(u,y);
  if(x==y) return;
  if(u->parent[x]>u->parent[y]){
    int swap=x;
    x=y;
    y=swap;
  }
  u->parent[x]+=u->parent[y];
  u->parent[y]=x;
}

typedef long long int int64;

typedef struct edge{
  int a,b,c;
} edge;

int cmp(const void *a,const void *b){
  return ((edge *)a)->c-((edge *)b)->c;
}

void run(void){
  int n,m;
  scanf("%d%d",&n,&m);
  edge *e=(edge *)calloc(m,sizeof(edge));
  int i;
  for(i=0;i<m;i++){
    int a,b,c;
    scanf("%d%d%d",&a,&b,&c);
    a--;b--;
    e[i]=(edge){a,b,c};
  }
  qsort(e,m,sizeof(edge),cmp);
  UnionFind *u=newUnionFind(n);
  int *sub=(int *)calloc(n*n,sizeof(int));
  int *a=(int *)calloc(n,sizeof(int));
  int *b=(int *)calloc(n,sizeof(int));
  int64 sum=0;
  for(i=0;i<m;i++){
    if(same(u,e[i].a,e[i].b)) continue;
    int alen=0;
    int blen=0;
    for(int j=0;j<n;j++){
      if(same(u,e[i].a,j)) a[alen++]=j;
      if(same(u,e[i].b,j)) b[blen++]=j;
    }
    for(int j=0;j<alen;j++){
      for(int k=0;k<blen;k++){
	int v=a[j];
	int u=b[k];
	sub[v*n+u]=e[i].c;
	sub[u*n+v]=e[i].c;
      }
    }
    sum+=e[i].c;
    unite(u,e[i].a,e[i].b);
  }
  int q;
  scanf("%d",&q);
  while(q--){
    int s,t;
    scanf("%d%d",&s,&t);
    s--;t--;
    printf("%lld\n",sum-sub[s*n+t]);
  }
}

int main(void){
  run();
  return 0;
}
