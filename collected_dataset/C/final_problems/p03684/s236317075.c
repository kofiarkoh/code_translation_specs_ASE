#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>
#define LL long long
#define INF 1<<31-1
#define MIN(x,y) ((x)<(y)?(x):(y))
#define MAX(x,y) ((x)<(y)?(y):(x))
#define FOR(i,a,n) for(i=a;i<n;i++)
#define MOD 1000000007
#define NUM 100001
void swap(int* a,int* b){
  int tmp=*a;*a=*b;*b=tmp;
}
int cmp(const void* a,const void* b){
  return *(int*)a-*(int*)b;
}

typedef struct{int s,e,c;}Edge;
int *par;
void UF(){
	par=calloc(NUM,4);
	for(int i=0;i<NUM;i++)par[i]=-1;
}
int Root(int x){
  if(par[x]<0)return x;
  else return par[x]=Root(par[x]);
}
int Size(int x){
  return -par[Root(x)];
}
bool Same(int x,int y){//x番目のノードとy番目のノードが同じ木に属しているか返す
  return Root(x)==Root(y);
}
void Unite(int x,int y){
  x=Root(x);y=Root(y);
  if(x!=y){
    if(Size(x)<Size(y))swap(&x,&y);
    par[x]+=par[y];par[y]=x;
  }
}
void Del(){
  free(par);
}
int cmpc(const void* a,const void* b){
	return ((Edge*)a)->c-((Edge*)b)->c;
};
int Kruscal(Edge* edge,int n){
  int i,ans=0;
  Edge e;
  qsort(edge,n,sizeof(Edge),cmpc);
  UF();
  FOR(i,0,n){
    e=edge[i];
    if(!Same(e.s,e.e)){
      Unite(e.s,e.e);
      ans+=e.c;
    }
  }
  Del();
  return ans;
}
Edge e[NUM*2];

typedef struct{int x,y,n;}P;
int cmpx(const void* a,const void* b){
  return ((P*)a)->x-((P*)b)->x;
}
int cmpy(const void *a,const void *b){
	return ((P*)a)->y-((P*)b)->y;
}
P p[NUM];

int main() {
  int N,i;
  scanf("%d",&N);
  FOR(i,0,N)scanf("%d%d",&p[i].x,&p[i].y);
  FOR(i,0,N)p[i].n=i;
  qsort(p,N,sizeof(P),cmpx);
  FOR(i,0,N-1)e[i].s=p[i].n,e[i].e=p[i+1].n,e[i].c=p[i+1].x-p[i].x;
  qsort(p,N,sizeof(P),cmpy);
  FOR(i,0,N-1)e[i+N-1].s=p[i].n,e[i+N-1].e=p[i+1].n,e[i+N-1].c=p[i+1].y-p[i].y;

  printf("%d\n",Kruscal(e,2*N-2));
	return 0;
}
