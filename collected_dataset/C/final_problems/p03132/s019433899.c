#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

#define POS(i,j) ((i)*5+(j))

int64 calc(const int *a,int n){
  int64 *dp=(int64 *)calloc(5*2,sizeof(int64));
  int now=0;
  for(int i=0;i<n;i++){
    int next=now^1;
    int v=a[i];
    if(v>0){
      dp[POS(next,0)]=dp[POS(now,0)]+v;
      dp[POS(next,1)]=MIN(dp[POS(now,0)],dp[POS(now,1)])+(v&1);
      dp[POS(next,2)]=MIN(dp[POS(now,0)],MIN(dp[POS(now,1)],dp[POS(now,2)]))+((v+1)&1);
      dp[POS(next,3)]=MIN(dp[POS(now,0)],MIN(dp[POS(now,1)],MIN(dp[POS(now,2)],dp[POS(now,3)])))+(v&1);
      dp[POS(next,4)]=MIN(dp[POS(now,0)],MIN(dp[POS(now,1)],MIN(dp[POS(now,2)],MIN(dp[POS(now,3)],dp[POS(now,4)]))))+v;
    } else {
      dp[POS(next,0)]=dp[POS(now,0)]+v;
      dp[POS(next,1)]=MIN(dp[POS(now,0)],dp[POS(now,1)])+2;
      dp[POS(next,2)]=MIN(dp[POS(now,0)],MIN(dp[POS(now,1)],dp[POS(now,2)]))+1;
      dp[POS(next,3)]=MIN(dp[POS(now,0)],MIN(dp[POS(now,1)],MIN(dp[POS(now,2)],dp[POS(now,3)])))+2;
      dp[POS(next,4)]=MIN(dp[POS(now,0)],MIN(dp[POS(now,1)],MIN(dp[POS(now,2)],MIN(dp[POS(now,3)],dp[POS(now,4)]))))+v;
    }
    now=next;
  }
  int64 res=dp[POS(now,0)];
  for(int i=1;i<5;i++) res=MIN(res,dp[POS(now,i)]);
  return res;
}

void run(void){
  int n;
  scanf("%d",&n);
  int *a=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",a+i);
  printf("%lld\n",calc(a,n));
}

int main(void){
  run();
  return 0;
}
