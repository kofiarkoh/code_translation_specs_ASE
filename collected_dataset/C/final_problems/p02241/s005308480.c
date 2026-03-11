#include<stdio.h>
#define MAX 100
#define INFTYtw 10000
#define WHITEtw 0
#define GRAYtw 1
#define BLACKtw 2

int n,Mtw[MAX][MAX];

int prim(){
  int i,v,u,minv;
  int d[MAX],p[MAX],color[MAX];

  for(i=0;i<n;i++){
    d[i]=INFTYtw;
    p[i]=-1;
    color[i]=WHITEtw;
  }

  d[0]=0;

  while(1){
    minv=INFTYtw;
    u=-1;
    for(i=0;i<n;i++){
      if(minv>d[i]&&color[i]!=BLACKtw){
	u=i;
	minv=d[i];
      }
    }
    if(u==-1)break;
    color[u]=BLACKtw;
    for(v=0;v<n;v++){
      if(color[v]!=BLACKtw&&Mtw[u][v]!=INFTYtw){
	if(d[v]>Mtw[u][v]){
	  d[v]=Mtw[u][v];
	  p[v]=u;
	  color[v]=GRAYtw;
	}
      }
    }
  }
  int sum=0;
  for(i=0;i<n;i++){
    if(p[i]!=-1)sum+=Mtw[i][p[i]];
  }

  return sum;
}

int main(){
  int i,j;
  scanf("%d",&n);

  for(i=0;i<n;i++){
    for(j=0;j<n;j++){
      int e;
      scanf("%d",&e);
      if(e==-1)Mtw[i][j]=INFTYtw;
      else Mtw[i][j]=e;
    }
  }

  printf("%d\n",prim());
  return 0;
}

