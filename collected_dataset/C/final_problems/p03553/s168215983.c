#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

int64 calc(int index,int n,int *a){
  if(3*index>n){
    int64 s=0;
    int i;
    for(i=1;i<=n;i++) s+=a[i];
    return s;
  }
  int64 res=calc(index+1,n,a);
  if(a[index]<0){
    int b[101];
    int i;
    for(i=index;i<=n;i+=index){
      b[i]=a[i];
      a[i]=0;
    }
    res=MAX(res,calc(index+1,n,a));
    for(i=index;i<=n;i+=index){
      a[i]=b[i];
    }
  }
  return res;
}

void run(void){
  int n;
  scanf("%d",&n);
  int a[101];
  int i;
  for(i=1;i<=n;i++) scanf("%d",a+i);
  for(i=n;3*i>n;i--){
    if(a[i]>=0) continue;
    int64 s=0;
    int j;
    for(j=i;j<=n;j+=i){
      s+=a[j];
    }
    if(s<=0){
      for(j=i;j<=n;j+=i){
	a[j]=0;
      }
    }
  }
  int64 ans=calc(1,n,a);
  printf("%lld\n",ans);
  return;
}


int main(void){
  run();
  return 0;
}
