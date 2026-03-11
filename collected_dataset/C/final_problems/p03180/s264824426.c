#include<stdio.h>
#include<stdlib.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))

void run(void){
  int n;
  scanf("%d",&n);
  int a[256];
  int i,j,k;
  for(i=0;i<n*n;i++) scanf("%d",a+i);
  int64 *dp=(int64 *)calloc(1<<n,sizeof(int64));
  for(i=1;i<(1<<n);i++){
    int64 local=0;
    for(j=0;j<n;j++){
      if(((i>>j)&1)==0) continue;
      for(k=j+1;k<n;k++){
	if(((i>>k)&1)==0) continue;
	local+=a[j*n+k];
      }
    }
    for(j=(i-1)&i;j>0;j=(j-1)&i){
      local=MAX(local,dp[j]+dp[i^j]);
    }
    dp[i]=local;
  }
  printf("%lld\n",dp[(1<<n)-1]);
}

int main(void){
  run();
  return 0;
}
