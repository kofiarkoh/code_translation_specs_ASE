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
  int x[100];
  int y[100];
  int i;
  for(i=0;i<n;i++) scanf("%d%d",x+i,y+i);
  int min=y[0];
  int index=0;
  for(i=1;i<n;i++){
    if(y[i]<min){
      min=y[i];
      index=i;
    }
  }
  if(min<0){
    int add=-min;
    for(i=0;i<n;i++) y[i]+=add;
  }
  int64 s=0;
  for(i=0;i+1<n;i++){
    s+=(x[i+1]-x[i])*(y[i]+y[i+1]);
  }
  s+=(x[0]-x[n-1])*(y[0]+y[n-1]);
  s=ABS(s);
  printf("%lld.%d\n",s/2,(s%2==0?0:5));
  return;
}

int main(void){
  run();
  return 0;
}

