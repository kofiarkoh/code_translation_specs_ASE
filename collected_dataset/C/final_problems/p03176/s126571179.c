#include<stdio.h>
#include<stdlib.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))

void update(int64 *bit,int x,int64 v){
  int n=(int)bit[0];
  for(int i=x;i<=n;i+=i&-i) bit[i]=MAX(bit[i],v);
}

int64 max(int64 *bit,int x){
  int64 res=0;
  for(int i=x;i>0;i-=i&-i) res=MAX(res,bit[i]);
  return res;
}

void run(void){
  int n;
  scanf("%d",&n);
  int *h=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",h+i);
  int *a=(int *)calloc(n,sizeof(int));
  for(i=0;i<n;i++) scanf("%d",a+i);
  int64 *bit=(int64 *)calloc(n+1,sizeof(int64));
  bit[0]=n;
  for(i=0;i<n;i++){
    if(h[i]==1){
      update(bit,1,a[i]);
    } else {
      int64 p=max(bit,h[i]-1);
      update(bit,h[i],p+a[i]);
    }
  }
  printf("%lld\n",max(bit,n));
}

int main(void){
  run();
  return 0;
}
