#include<stdio.h>
#include<stdlib.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))

void run(void){
  int n,e,t;
  scanf("%d%d%d",&n,&e,&t);
  if(n>2000) exit(1);
  int *x=(int *)calloc(n+1,sizeof(int));
  int i;
  for(i=1;i<=n;i++) scanf("%d",x+i);
  int64 *dp=(int64 *)calloc(n+1,sizeof(int64));
  for(i=1;i<=n;i++){
    int64 min=1000000000LL*3;
    int j;
    for(j=i;j>0;j--){
      int64 moveJ=dp[j-1]+x[j]-x[j-1];
      int64 d=x[i]-x[j];
      int64 v=moveJ+2*d+MAX(t-2*d,0)+d;
      min=MIN(min,v);
    }
    dp[i]=min;
  }
  int64 ans=dp[n]+e-x[n];
  printf("%lld\n",ans);
}

int main(void){
  run();
  return 0;
}
