#include<stdio.h>
int min(int a,int b){if(a>b) return b;return a;}
int main(){
  int h,n,i,j;
  scanf("%d%d",&h,&n);
  int a[n],b[n];
  for(i=0;i<n;i++)
    scanf("%d%d",&a[i],&b[i]);
  int dp[100100];
  for(i=1;i<100100;i++)
    dp[i]=10010010;
  dp[0]=0;
  for(i=0;i<n;i++){
    for(j=0;j<h;j++){
      dp[min(h,(j+a[i]))]=min(dp[min(h,(j+a[i]))],dp[j]+b[i]);
    }
  }
  printf("%d\n",dp[h]);
  return 0;}
