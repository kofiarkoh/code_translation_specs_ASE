#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

void run(void){
  int n;
  scanf("%d",&n);
  int *a=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",a+i);
  int64 *dp=(int64 *)calloc(n,sizeof(int64));
  int *next=(int *)calloc(n,sizeof(int));
  dp[n-1]=0;
  next[n-1]=n;
  for(i=n-2;i>=0;i--){
    dp[i]=dp[i+1];
    next[i]=next[i+1];
    if(a[i]<=a[i+1]){
      if(a[i]*4LL<=a[i+1]){
	next[i]=i+1;
      }
      continue;
    }
    int cnt=0;
    int p;
    for(p=i+1;p<n;p=next[p]){
      double prev=a[p-1]*pow(4,cnt);
      if(prev<=a[p]){
	while(cnt--){
	  a[p-1]*=4;
	}
	break;
      }
      cnt=0;
      double t=a[p];
      while(prev>t){
	t*=4;
	cnt++;
      }
      dp[i]+=2LL*cnt*(next[p]-1-i);
    }
    next[i]=p;
  }
  for(i=0;i<n-1-i;i++){
    int swap=a[i];
    a[i]=a[n-1-i];
    a[n-1-i]=swap;
  }
  int64 *dp2=(int64 *)calloc(n,sizeof(int64));
  dp2[n-1]=1;
  next[n-1]=n;
  for(i=n-2;i>=0;i--){
    dp2[i]=dp2[i+1]+1;
    next[i]=next[i+1];
    if(a[i]<=a[i+1]){
      if(a[i]*4LL<=a[i+1]){
	next[i]=i+1;
      }
      continue;
    }
    int cnt=0;
    int p;
    for(p=i+1;p<n;p=next[p]){
      double prev=a[p-1]*pow(4,cnt);
      if(prev<=a[p]){
	while(cnt--){
	  a[p-1]*=4;
	}
	break;
      }
      cnt=0;
      double t=a[p];
      while(prev>t){
	t*=4;
	cnt++;
      }
      dp2[i]+=2LL*cnt*(next[p]-1-i);
    }
    next[i]=p;
  }
  int64 min=dp[0];
  for(i=1;i<n;i++){
    int64 x=dp[i]+dp2[n-i];
    if(x<min) min=x;
  }
  printf("%lld\n",min);
}

void naive(void){
  int n;
  scanf("%d",&n);
  int *a=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",a+i);
  int64 min=10000000000000;
  int64 *b=(int64 *)calloc(n,sizeof(int64));
  for(i=0;i<n;i++){
    int64 cnt=0;
    b[i]=a[i];
    int j;
    for(j=i+1;j<n;j++){
      b[j]=a[j];
      while(b[j]<b[j-1]){
	b[j]*=4;
	cnt+=2;
      }
    }
    if(i>0){
      b[i-1]=-2*a[i-1];
      cnt++;
      for(j=i-2;j>=0;j--){
	b[j]=-2*a[j];
	while(b[j]>b[j+1]){
	  b[j]*=4;
	  cnt+=2;
	}
      }
    }
    if(cnt<min) min=cnt;
  }
  printf("%lld\n",min);
}

int main(void){
  run();
  return 0;
}
