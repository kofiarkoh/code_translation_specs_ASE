#include<stdio.h>
#define INF 10000000
int prim(void);
int min(int,int);
int n,graph[101][101],used[101],mincost[101];

int main(void){
  int m,s,g1,g2,b1,b2,c,i,j;
  
    while(scanf("%d %d %d %d %d",&n,&m,&s,&g1,&g2),n,m,s,g1,g2){
      
      for(i=0;i<=n;i++){
	for(j=0;j<=n;j++){
	  graph[i][j]=INF;
	}
	used[i]=0;
	mincost[i]=INF;
      }
      
      for(i=0;i<m;i++){
	scanf("%d %d %d",&b1,&b2,&c);
	graph[b1][b2]=c;
      }

      printf("%d\n",prim());
    }
    
    return 0;
}

int prim(void){
  int i,v,rev=0;
  mincost[1]=0;
  while(1){
    v=-1;
    for(i=1;i<=n;i++){
      if(!used[i] && (v==-1 || mincost[i]<mincost[v]))v=i;
    }
    
    if(v==-1)break;
    used[v]=1;
    rev+=mincost[v];

    for(i=1;i<=n;i++){
      mincost[i]=min(mincost[i],graph[v][i]);
    }
  }
  return rev;
}

int min(int x,int y){
  return (x<y)?x:y;
}