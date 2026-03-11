#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

int64 calc(int *a,int *b,int n,int k){
  int64 sum=0;
  int i;
  for(i=0;i<n;i++) if((a[i]|k)==k) sum+=b[i];
  return sum;
}

void run(void){
  int n,k;
  scanf("%d%d",&n,&k);
  int *a=(int *)malloc(sizeof(int)*n);
  int *b=(int *)malloc(sizeof(int)*n);
  int i;
  for(i=0;i<n;i++) scanf("%d%d",a+i,b+i);
  int64 max=calc(a,b,n,k);
  int bit;
  for(bit=0;bit<30;bit++){
    if((k>>bit)&0x01){
      int64 t=calc(a,b,n,((k>>(bit+1))<<(bit+1))+(1<<bit)-1);
      max=MAX(max,t);
    }
  }
  printf("%lld\n",max);
  return;
}

int main(void){
  run();
  return 0;
}