#include<stdio.h>
int mx(int a,int b){
  if(a<b)return b;
  else   return a;
}
int main(){
  int N,W,c=1,mi,max,i,j;
  int k[1001],w[1001];
  while(1){
    scanf("%d",&W);
    if(!W)break;
    scanf("%d",&N);
    int dp[1001][1001]={0};
    for(i=0;i<N;i++){
      scanf("%d,%d",&k[i],&w[i]);
    }
    for(i=N-1;i>=0;i--){
      for(j=0;j<=W;j++){
	if(j<w[i])dp[i][j]=dp[i+1][j];
	else{
	  dp[i][j]=mx(dp[i+1][j],dp[i+1][j-w[i]]+k[i]);
	}
      }
    }
    max=0;
    mi=0;
    for(i=0;i<W;i++){
      if(max<dp[0][i]){
	max=dp[0][i];
	mi=i;
      }
    }
    printf("Case %d:\n%d\n%d\n",c++,max,mi);
  }
  return 0;
}