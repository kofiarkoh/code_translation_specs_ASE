#include<stdio.h>
#include<stdlib.h>
int main(){
  int i,n,a[100010],dp[100010];
  scanf("%d\n",&n);
  for(i=1;i<=n;i++){
    scanf("%d\n",&a[i]);
  }
  for(i=0;i<=n;i++){
    dp[i]=100000;
  }
  dp[1]=0;
  for(i=2;i<=n;i++){
    if(abs(a[i-1]-a[i])+dp[i-1]<dp[i]){
      dp[i]=abs(a[i-1]-a[i])+dp[i-1];
    }
    if(i!=2){
    if(abs(a[i-2]-a[i])+dp[i-2]<dp[i]){
      dp[i]=abs(a[i-2]-a[i])+dp[i-2];
    }
    }
  }
  printf("%d\n",dp[n]);
  return 0;
}