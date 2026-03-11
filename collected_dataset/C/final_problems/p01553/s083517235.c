#include<stdio.h>
#define Prime 1000000007
int main(void){
  int n,i,j;
int cnt;
  char c[202][20];
  long long dp[202][202]={{1}};
  
  scanf("%d",&n);
  
  for(i=0;i<n;i++)
    scanf("%s",&c[i]);
cnt=0;
for(i=0;i<n;i++){
if(c[i][0]=='-')cnt++;
for(j=0;j<n+1;j++){
    if(c[i][0]=='D'){
      dp[i-cnt+1][j-cnt]+=dp[i-cnt][j-cnt]*(j-cnt);
      if(j)dp[i+1-cnt][j-1-cnt]+=dp[i-cnt][j-cnt]*(j-cnt)*(j-cnt);
      dp[i+1-cnt][j-cnt]%=Prime;
      if(j)dp[i+1-cnt][j-1-cnt]%=Prime;
    }else if(c[i][0]=='U'){
      dp[i+1-cnt][j+1-cnt]+=dp[i-cnt][j-cnt];
      dp[i+1-cnt][j-cnt]+=dp[i-cnt][j-cnt]*(j-cnt);
      dp[i+1-cnt][j+1-cnt]%=Prime;
      dp[i+1-cnt][j-cnt]%=Prime;
    }
  }}
  printf("%lld\n",dp[i-cnt][0]);
  return 0;}