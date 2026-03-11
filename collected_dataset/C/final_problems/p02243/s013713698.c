#include<stdio.h>
#include<stdlib.h>
#define INF 10000000
#define M 10001
#define NIL -1
#define W 0
#define G 1
#define B 2
struct node{
  int k,c;
  struct node *next;
};

typedef struct node * nodepointer;

nodepointer sssp[M];
nodepointer mknd(int,int,nodepointer);
void insert(int,int,int);

int main(){
  int i,j,k,n,u,v,c,clr[M],dt[M],t[M],min,x;
  nodepointer a;

  scanf("%d",&n);
  for(i=0;i<n;i++) sssp[i] = mknd(0,0,NULL);
  for(i=0;i<n;i++){
    scanf("%d",&u);
    scanf("%d",&k);
    for(j=0;j<k;j++){
      scanf("%d",&v);
      scanf("%d",&c);
      insert(i,v,c);
      }
  }
  for(i=0;i<n;i++){
    clr[i]=W;
    t[i]=NIL;
    dt[i] = INF;

  }
  dt[0]=0;
  while(1){
    min=INF;
    for(i=0;i<n;i++){
      if(clr[i] != B && dt[i]<min){
	min =dt[i];
	x=i;
      }

    }
    if(min==INF) break;
    clr[x]=B;

    for(a=sssp[x];a!=NULL;a=a->next){
      if(clr[a->k]!=B && (a->c + dt[x])<dt[a->k]){
	t[a->k]=x;

	dt[a->k]=a->c + dt[x];
	clr[a->k] = G;
      }
    }
  }

  for(i=0;i<n;i++) printf("%d %d\n",i,dt[i]);
  

  return 0;
}
nodepointer mknd(int k,int c,nodepointer node){
  nodepointer b;
  b=malloc(sizeof(struct node));
  b->k=k;
  b->c=c;
  b->next = node;

  
  return b;
}


void insert(int i,int x,int y){
  nodepointer mn;
  mn=mknd(x,y,sssp[i]->next);
  sssp[i]->next = mn;

}

