#include<stdio.h>
#include<stdlib.h>
long minf(long a,long b){
  if(a>b){
    return b;
  }else{
    return a;
  }
}

long maxf(long a,long b){
  if(a>b){
    return a;
  }else{
    return b;
  }
}

int main(){
  long n = 0;
  long a[100000][3] = {};
  long dp[100000][3] = {};
  long k = 0;
  long i = 0;
  long j = 0;
  long min = -1;
  long max = -1;
  
  scanf("%ld",&n);
  
  for(i=0;i<n;i++){
    scanf("%ld %ld %ld",&a[i][0],&a[i][1],&a[i][2]);
  }
  
  dp[0][0] = a[0][0];
  dp[0][1] = a[0][1];
  dp[0][2] = a[0][2];
  
  for(i=1;i<n;i++){
    for(j=0;j<3;j++){
      max = -1;
      for(k=0;k<3;k++){
        if(j != k){
          if(max == -1){
            max = dp[i-1][k] + a[i][j];
          }else{
            max = maxf(max,dp[i-1][k] + a[i][j]);
          }
        }
      }
      dp[i][j] = max;
      //printf("%ld.%ld-%ld,",max,i,j);
    }  
  }
  max = maxf(dp[n-1][0],dp[n-1][1]);
  max = maxf(max,dp[n-1][2]);
  printf("%ld",max);
  return 0;
}
    