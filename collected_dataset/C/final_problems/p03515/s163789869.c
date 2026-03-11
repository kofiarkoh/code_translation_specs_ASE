#include<stdio.h>
#include<stdlib.h>

typedef long long int int64;

typedef struct UnionFind{
  int *parent;
  int64 *add;
  int n;
} UnionFind;

UnionFind* newUnionFind(int n){
  UnionFind *u=(UnionFind *)calloc(1,sizeof(UnionFind));
  u->parent=(int *)calloc(n,sizeof(int));
  u->add=(int64 *)calloc(n,sizeof(int64));
  u->n=n;
  for(int i=0;i<n;i++) u->parent[i]=-1;
  return u;
}

int root(UnionFind *u,int x){
  while(u->parent[x]>=0) x=u->parent[x];
  return x;
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
  u->add[y]-=u->add[x];
}

void addVal(UnionFind *u,int x,int64 v){
  x=root(u,x);
  u->add[x]+=v;
}

int64 getVal(UnionFind *u,int x){
  int64 sum=0;
  while(x>=0){
    sum+=u->add[x];
    x=u->parent[x];
  }
  return sum;
}

typedef struct weightEdge{
  int a,b,c;
} edge;

int cmpEdge(const void *a,const void *b){
  return ((edge *)a)->c-((edge *)b)->c;
}

void run(void){
  int n;
  scanf("%d",&n);
  edge *e=(edge *)calloc(n-1,sizeof(edge));
  int i;
  for(i=0;i<n-1;i++){
    int a,b,c;
    scanf("%d%d%d",&a,&b,&c);
    a--;b--;
    e[i]=(edge){a,b,c};
  }
  qsort(e,n-1,sizeof(edge),cmpEdge);
  UnionFind *u=newUnionFind(n);
  for(i=n-2;i>=0;i--){
    addVal(u,e[i].a,(int64)e[i].c*getSize(u,e[i].b));
    addVal(u,e[i].b,(int64)e[i].c*getSize(u,e[i].a));
    unite(u,e[i].a,e[i].b);
  }
  for(i=0;i<n;i++) printf("%lld\n",getVal(u,i));
}

int main(void){
  run();
  return 0;
}
