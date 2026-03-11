#include<stdio.h>
#include<stdlib.h>

typedef long long int int64;

void run(void){
  int n,q;
  scanf("%d%d",&n,&q);
  int *a=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",a+i);
  while(q--){
    int64 x;
    scanf("%lld",&x);
    int64 ans=0;
    int j=0;
    for(i=0;i<n;i++){
      while(j<n && x-a[j]>=0) x-=a[j++];
      ans+=j-i;
      x+=a[i];
    }
    printf("%lld\n",ans);
  }
}

int main(void){
  run();
  return 0;
}

