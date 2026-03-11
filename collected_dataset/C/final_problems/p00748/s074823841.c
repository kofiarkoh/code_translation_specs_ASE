#include <stdio.h>
#include <stdlib.h>
#define num 100000
#define true 1
#define false 0

void pollock();
int min(int f1,int f2);
int *dp_ki;
int *dp;
int N;

int main(){
  dp_ki = (int *)malloc(num * sizeof(int));
  dp = (int *)malloc(num * sizeof(int));
  for(int i=0;i<num;i++){
    dp[i]=i;
    dp_ki[i]=i;
  }
  pollock();

  while(scanf("%d",&N),N){
    printf("%d %d\n",dp[N],dp_ki[N] );
  }

  free(dp_ki);
  free(dp);
  exit(0);
}

int min(int f1,int f2){
  if(f1>f2){
    return f2;
  }else{
    return f1;
  }
}

void pollock(){
  for(int n=3,p=4;p<num;n++){
    for(int i=0;i+p<num;i++){
      dp[i+p] = min(dp[i+p],dp[i]+1);
    }
    if(p%2){
      for(int i=0;i+p<num;i++){
        dp_ki[i+p] = min(dp_ki[i+p],dp_ki[i]+1);
      }
    }
    p= n*(n+1)*(n+2)/6;
  }
}

