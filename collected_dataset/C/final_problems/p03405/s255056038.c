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

void initUnionFind(UnionFind *u){
  for(int i=0;i<u->n;i++) u->parent[i]=-1;
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

const int mod=1000000007;

int modPow(int r,int n){
  int t=1;
  int s=r;
  while(n>0){
    if(n&1) t=(int64)t*s%mod;
    s=(int64)s*s%mod;
    n>>=1;
  }
  return t;
}

typedef struct edge{
  int a,b,c;
} edge;

int cmpEdge(const void *a,const void *b){
  return ((edge *)a)->c-((edge *)b)->c;
}

void run(void){
  int n,m;
  int64 x;
  scanf("%d%d%lld",&n,&m,&x);
  edge *e=(edge *)calloc(m,sizeof(edge));
  int i;
  for(i=0;i<m;i++){
    int a,b,c;
    scanf("%d%d%d",&a,&b,&c);
    e[i]=(edge){a-1,b-1,c};
  }
  qsort(e,m,sizeof(edge),cmpEdge);
  UnionFind *u=newUnionFind(n);
  int ans=0;
  for(i=1;i<m;i++){
    initUnionFind(u);
    unite(u,e[i].a,e[i].b);
    int64 sum=e[i].c;
    for(int j=0;j<m;j++){
      if(same(u,e[j].a,e[j].b)) continue;      
      unite(u,e[j].a,e[j].b);
      sum+=e[j].c;
    }
    if(sum==x){
      ans=(ans+modPow(2,m-i))%mod;
    }
  }
  printf("%d\n",ans);
}

int main(void){
  run();
  return 0;
}
