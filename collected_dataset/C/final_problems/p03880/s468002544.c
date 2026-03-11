#include<stdio.h>
#include<stdlib.h>

void run(void){
  int n;
  scanf("%d",&n);
  int *a=(int *)calloc(n,sizeof(int));
  int *used=(int *)calloc(n,sizeof(int));
  int i;
  for(i=0;i<n;i++) scanf("%d",a+i);
  int g=0;
  for(i=0;i<n;i++) g^=a[i];
  int cnt=0;
  while(g>0){
    int b=1;
    while(2*b<=g) b*=2;
    for(i=0;i<n;i++) if(!used[i] && a[i]%(2*b)==b) break;
    if(i>=n){
      printf("-1\n");
      return;
    }
    g=g^a[i]^(a[i]-1);
    a[i]--;
    used[i]=1;
    cnt++;
  }
  printf("%d\n",cnt);
}

int main(void){
  run();
  return 0;
}
