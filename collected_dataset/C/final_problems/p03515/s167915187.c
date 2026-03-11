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

typedef struct weightEdge{
  int a,b;
  int64 c;
} edge;

int cmpEdge(const void *a,const void *b){
  int64 p=((edge *)a)->c;
  int64 q=((edge *)b)->c;
  return p==q?0:p<q?1:-1;
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
  int *r=(int *)calloc(n,sizeof(int));
  for(i=0;i<n;i++) r[i]=i;
  edge *binaryTreeEdge=(edge *)calloc(2*n,sizeof(edge));
  int len=0;
  UnionFind *u=newUnionFind(n);
  for(i=0;i<n-1;i++){
    int a=e[i].a;
    int b=e[i].b;
    int64 c=e[i].c;
    int64 sa=getSize(u,a);
    int64 sb=getSize(u,b);
    binaryTreeEdge[len++]=(edge){n+i,r[root(u,a)],sb*c};
    binaryTreeEdge[len++]=(edge){n+i,r[root(u,b)],sa*c};
    unite(u,a,b);
    r[root(u,a)]=n+i;
  }
  int64 *dp=(int64 *)calloc(2*n,sizeof(int64));
  for(i=len-1;i>=0;i--){
    int from=binaryTreeEdge[i].a;
    int to=binaryTreeEdge[i].b;
    int weight=binaryTreeEdge[i].c;
    dp[to]=dp[from]+weight;
  }
  for(i=0;i<n;i++) printf("%lld\n",dp[i]);
}

int main(void){
  run();
  return 0;
}
