#include<stdio.h>
#include<stdlib.h>

#define POS(i,j) (((i)&1)*n+(j))

const int mod=1000000007;

void run(void){
  int n;
  scanf("%d",&n);
  char *s=(char *)calloc(n,sizeof(char));
  scanf("%s",s);
  int *dp=(int *)calloc(2*n,sizeof(int));
  int i,j;
  for(i=0;i<n;i++) dp[POS(0,i)]=1;
  for(i=1;i<n;i++){
    for(j=0;j<n;j++) dp[POS(i,j)]=0;
    if(s[i-1]=='<'){
      for(j=0;j<n-i;j++){
	dp[POS(i,j)]+=dp[POS(i-1,j)];
	dp[POS(i,j)]%=mod;
      }
    } else {
      for(j=1;j<n+1-i;j++){
	dp[POS(i,0)]+=dp[POS(i-1,j)];
	dp[POS(i,0)]%=mod;
	dp[POS(i,j)]+=mod-dp[POS(i-1,j)];
	dp[POS(i,j)]%=mod;
      }
    }
    for(j=1;j<n-i;j++){
      dp[POS(i,j)]+=dp[POS(i,j-1)];
      dp[POS(i,j)]%=mod;
    }
  }
  printf("%d\n",dp[POS(n-1,0)]);
}

int main(void){
  run();
  return 0;
}
