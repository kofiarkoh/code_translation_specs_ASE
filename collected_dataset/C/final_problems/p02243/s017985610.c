#include<stdio.h>
#include<stdlib.h>
#define N 10001
#define M 10000001
struct node{
  int key;
  int cost;
  struct node *next;
};
typedef struct node *GG;
 
GG G[10001];
GG MNode(int,int,GG);
void Insert(int,int,int);
int main()
{
  int i,j,n,count=0,v,c,c1,v1;
  int color[N],d[N],pi[N],mincost,ii;
  GG GGG;
  scanf("%d",&n);
  for(i=0;i<n;i++)G[i]=MNode(0,0,NULL);
  for(i=0;i<n;i++){
    scanf("%d%d",&v,&c);
    for(j=0;j<c;j++){
      scanf("%d%d",&v1,&c1);
      Insert(i,v1,c1);
    }
  }
  for(i=0;i<n;i++){
    color[i]=0;
    pi[i]=-1;
    d[i]=M;
  }
  d[0]=0;
  
  while(1){
    mincost=M;
    for(i=0;i<n;i++){
      if(color[i]!=2 && d[i]<mincost){
        mincost=d[i];
        ii=i;
      }
    }
    if(mincost==M)break;
    color[ii]=2;
    for(GGG=G[ii];GGG!=NULL;GGG=GGG->next){
      if(color[GGG->key]!=2 && (GGG->cost+d[ii])<d[GGG->key]){
	pi[GGG->key]=ii;
	d[GGG->key]=GGG->cost+d[ii];
	color[GGG->key]=1;
      }
    }
  }
  for(i=0;i<n;i++)printf("%d %d\n",i,d[i]);
  
  return 0;
}
 
GG MNode(int key,int cost,GG Node)
{
  GG n;
  n=(GG)malloc(sizeof(struct node));
  n->key=key;
  n->cost=cost;
  n->next=Node;
  return n;
}
void Insert(int s,int t,int u)
{
  GG GGG;
  GGG=MNode(t,u,G[s]->next);
  G[s]->next=GGG;
}

