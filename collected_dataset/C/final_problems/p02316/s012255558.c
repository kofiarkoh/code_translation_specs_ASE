#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

void run(void){
  int n,w;
  scanf("%d%d",&n,&w);
  int val[100];
  int weight[100];
  int i;
  for(i=0;i<n;i++){
    scanf("%d%d",val+i,weight+i);
  }
  int dp[10001];
  for(i=0;i<=w;i++) dp[i]=-1;
  dp[0]=0;
  for(i=0;i<n;i++){
    int j;
    for(j=0;j<=w-weight[i];j++){
      if(dp[j]>=0){
	int k=j+weight[i];
	dp[k]=MAX(dp[k],dp[j]+val[i]);
      }
    }
  }
  int ans=0;
  for(i=0;i<=w;i++){
    ans=MAX(ans,dp[i]);
  }
  printf("%d\n",ans);
  return;
}

int main(void){
  run();
  return 0;
}
