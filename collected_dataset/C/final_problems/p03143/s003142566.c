#include<stdio.h>
#include<stdlib.h>

typedef long long int int64;

typedef int EdgeWeight;

typedef struct directedGraph{
  int *vertex;
  EdgeWeight *cost;
  int *next;
  int *start;
  int v,e;
  int pointer;
} graph;

graph* newGraph(const int v,const int e){
  graph *g=(graph *)malloc(sizeof(graph));
  g->vertex=(int *)calloc(e,sizeof(int));
  g->cost=(EdgeWeight *)calloc(e,sizeof(EdgeWeight));
  g->next=(int *)calloc(e,sizeof(int));
  g->start=(int *)calloc(v,sizeof(int));
  for(int i=0;i<v;i++) g->start[i]=-1;
  g->v=v;
  g->e=e;
  g->pointer=0;
  return g;
}

void addEdge(graph *g,const int from,const int to,const EdgeWeight cost){
  const int p=g->pointer;
  g->vertex[p]=to;
  g->cost[p]=cost;
  g->next[p]=g->start[from];
  g->start[from]=p;
  g->pointer++;
  return;
}

typedef struct edge{
  int a,b,c;
} edge;

int cmpEdge(const void *a,const void *b){
  return ((edge *)a)->c-((edge *)b)->c;
}

typedef struct UnionFind{
  int *parent;
  int64 *weight;
  int n;
} UnionFind;

UnionFind* newUnionFind(int n,int *x){
  UnionFind *u=(UnionFind *)calloc(1,sizeof(UnionFind));
  u->parent=(int *)calloc(n,sizeof(int));
  u->weight=(int64 *)calloc(n,sizeof(int64));
  int i;
  for(i=0;i<n;i++){
    u->parent[i]=i;
    u->weight[i]=x[i];
  }
  return u;
}

int root(UnionFind *u,int x){
  if(u->parent[x]==x) return x;
  return u->parent[x]=root(u,u->parent[x]);
}

int64 getWeight(UnionFind *u,int x){
  return u->weight[root(u,x)];
}

void unite(UnionFind *u,int x,int y){
  x=root(u,x);
  y=root(u,y);
  if(x==y) return;
  u->parent[x]=y;
  u->weight[y]+=u->weight[x];
}

void run(void){
  int n,m;
  scanf("%d%d",&n,&m);
  int *x=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",x+i);
  edge *e=(edge *)calloc(m,sizeof(edge));
  graph *g=newGraph(n,2*m);
  for(i=0;i<m;i++){
    int a,b,c;
    scanf("%d%d%d",&a,&b,&c);
    a--;b--;
    addEdge(g,a,b,c);
    addEdge(g,b,a,c);
    e[i]=(edge){a,b,c};
  }
  qsort(e,m,sizeof(edge),cmpEdge);
  UnionFind *u=newUnionFind(n,x);
  int64 *weight=(int64 *)calloc(m,sizeof(int64));
  for(i=0;i<m;i++){
    unite(u,e[i].a,e[i].b);
    weight[i]=getWeight(u,e[i].a);
  }
  int cnt=0;
  int *ok=(int *)calloc(n,sizeof(int));
  int *q=(int *)calloc(n,sizeof(int));
  for(i=m-1;i>=0;i--){
    if(ok[e[i].a]) continue;
    int cost=e[i].c;
    if(cost<=weight[i]){
      int front=0,last=0;
      ok[e[i].a]=1;
      q[last++]=e[i].a;
      while(front<last){
	int v=q[front++];
	for(int p=g->start[v];p!=-1;p=g->next[p]){
	  int u=g->vertex[p];
	  int c=g->cost[p];
	  if(c>cost) continue;
	  if(ok[u]) continue;
	  ok[u]=1;
	  q[last++]=u;
	}
      }
    } else {
      cnt++;
    }
  }
  printf("%d\n",cnt);
}

int main(void){
  run();
  return 0;
}
