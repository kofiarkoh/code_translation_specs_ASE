#include<stdio.h>
#include<stdlib.h>

typedef long long int int64;

int64* calc(const int *a,const int n){
  const int f=16;
  int64 *now=(int64 *)calloc(f,sizeof(int64));
  int64 *next=(int64 *)calloc(f,sizeof(int64));
  int64 *dp=(int64 *)calloc(n,sizeof(int64));
  int i,j;
  for(i=0;i<f;i++) now[i]=2*i;
  dp[n-1]=now[0];
  for(i=n-2;i>=0;i--){
    int64 t=a[i];
    int64 v=a[i+1];
    int k=0;
    for(j=0;j<15;j++,t*=4){
      for(;t>v;k++,v*=4);
      if(k<f){
	next[j]=2*j+now[k];
      } else {
	next[j]=2*j+now[f-1]+2*(k-f+1)*(n-i-1);
      }
    }
    dp[i]=next[0];
    int64 *swap=now;
    now=next;
    next=swap;
  }
  free(now);
  free(next);
  return dp;
}

void run(void){
  int n;
  scanf("%d",&n);
  int *a=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",a+i);
  int64 *dp1=calc(a,n);
  for(i=0;i<n-1-i;i++){
    int swap=a[i];
    a[i]=a[n-1-i];
    a[n-1-i]=swap;
  }
  int64 *dp2=calc(a,n);
  int64 min=dp1[0];
  for(i=1;i<n;i++){
    int64 t=dp1[i]+dp2[n-i]+i;
    if(t<min) min=t;
  }
  printf("%lld\n",min);
  free(a);
  free(dp1);
  free(dp2);
}

int main(void){
  run();
  return 0;
}
