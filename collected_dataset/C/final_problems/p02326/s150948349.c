#include <stdio.h>
#define MAX 1400
int H,W;
int C[MAX][MAX];
int dp[MAX][MAX];
int min(int a,int b){
  if(a<b)return a;
  else return b;
}
int max(int a,int b){
  if(a<b)return b;
  else return a;
}
int largest(){
  int i,j;
  int maxwidth=0;
  for(i=0;i<H;i++){
    dp[i][0]=(C[i][0]==1)? 0 : 1;
  }
  
  for(j=0;j<W;j++){
    dp[0][j]=(C[0][j]==1)? 0 : 1;
  }
  for(i=1;i<H;i++){
    for(j=1;j<W;j++){
      if(C[i][j])dp[i][j]=0;
      else{
	dp[i][j]=min(dp[i-1][j-1],min(dp[i-1][j],dp[i][j-1]))+1;
	maxwidth=max(maxwidth,dp[i][j]);
      }
    }
  }
  return maxwidth*maxwidth;
}
int main(){
  int i,j;
  scanf("%d %d",&H,&W);
  for(i=0;i<H;i++){
    for(j=0;j<W;j++){
      scanf("%d",&C[i][j]);
    }
  }
  printf("%d\n",largest());
  return 0;
}