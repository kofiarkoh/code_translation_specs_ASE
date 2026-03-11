#include<stdio.h>
int main(){
  int s,d,m,i,j,k,l;
  int w[101][101],p[101][101],n[101];
  while(scanf("%d %d %d",&s,&d,&m)!=EOF){
    int dp[100][300]={0};
    for(i=0;i<d;i++){
      scanf("%d",&n[i]);
      for(j=0;j<n[i];j++){
	scanf("%d %d",&w[i][j],&p[i][j]);
      }
    }
    for(i=1;i<=s;i++){
      scanf("%d",&k);
      for(j=0;j<n[k];j++){
	for(l=0;l<=m;l++){
	  if(dp[i][l]==0||dp[i][l]<dp[i-1][l])dp[i][l]=dp[i-1][l];
	  if(l<p[k][j])continue;
	  if(dp[i][l]<dp[i-1][l-p[k][j]]+w[k][j]||dp[i][l]==0){
	     dp[i][l]=dp[i-1][l-p[k][j]]+w[k][j];
	  }
	}
      }
    }
    for(i=m;i;i--){
      if(dp[s][i]!=dp[s][i-1])break;
    }
    printf("%d %d\n",dp[s][i],i);
  }   
  return 0;
}