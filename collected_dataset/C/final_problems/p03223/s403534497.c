#include<stdio.h>
#include<stdlib.h>
#include<math.h>

typedef long long int int64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))

int cmp(const void *a,const void *b){
  return *(int *)a-*(int *)b;
}

void run(void){
  int n;
  scanf("%d",&n);
  int *a=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",a+i);
  qsort(a,n,sizeof(int),cmp);
  int64 ans=0;
  for(i=0;i<n-1-i;i++){
    ans+=2*(a[n-1-i]-a[i]);
  }
  if(n%2!=0){
    int m=n/2;
    ans-=MIN(a[m]-a[m-1],a[m+1]-a[m]);
  } else {
    ans-=a[n/2+1]-a[n/2];
  }
  printf("%lld\n",ans);
}

int main(void){
  run();
  return 0;
}
