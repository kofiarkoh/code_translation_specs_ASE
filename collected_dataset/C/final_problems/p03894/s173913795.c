#include<stdio.h>
#include<stdlib.h>

void run(void){
  int n,q;
  scanf("%d%d",&n,&q);
  int *p=(int *)calloc(n+2,sizeof(int));
  p[1]=p[2]=1;
  int now=1;
  while(q--){
    int a,b;
    scanf("%d%d",&a,&b);
    int s=p[a];
    p[a]=p[b];
    p[b]=s;
    if(now==a || now==b){
      now^=a^b;
    }
    p[now-1]=p[now+1]=1;
  }
  int ans=0;
  for(int i=1;i<=n;i++) ans+=p[i];
  printf("%d\n",ans);
}

int main(void){
  run();
  return 0;
}