#include<stdio.h>

#define N 100
#define WHITE 0
#define GRAY 1
#define BLACK 2
#define INFTY (1<<21)
int prim(void);
int n,A[N][N];

int main()
{
  int i,j,x;

  scanf("%d",&n);

  for(i=0;i<n;i++){
    for(j=0;j<n;j++){
      scanf("%d",&x);
      if(x==-1)A[i][j]=INFTY;
      else A[i][j]=x;}
                  }
  printf("%d\n",prim());
  return 0;
}
int prim(void)
{
  int color[N],d[N],p[N],s,i,j,mini,u;
  for(s=0;s<n;s++){
    color[s]=WHITE;
    d[s]=INFTY;
    p[s]=-1;
  }
  d[0]=0;
  while(1)
    {mini=INFTY;
      u=-1;
      for(i=0;i<n;i++){
	if(color[i]!=BLACK && d[i]<mini){
	  u=i;
	  mini=d[i];                  	}
                     }
      if(u==-1)break;

      color[u]=BLACK;

      for(i=0;i<n;i++){
	if(color[i]!=BLACK && A[u][i]!=INFTY)
	  if(A[u][i]<d[i]){
	    d[i]=A[u][i];
	    p[i]=u;
	    color[i]=GRAY;}}
   }
  int sum=0;
  for(i=0;i<n;i++){
    if(p[i]!=-1)sum+=A[i][p[i]];}
  
  return sum;
}

