#include<stdio.h>
#define N 10000
#define M 1010

int main(void){
  int i,num,j,k;
  int n,m;
  int p[M];
  int swp,sf;
  int ans[N];

  for(i=0;;i++){
    ans[i]=0;
    scanf("%d%d",&n,&m);
    if(n==0 && m==0) break;

    for(j=0;j<n;j++) scanf("%d",&p[j]);

    for(;;){
      sf=1;
      for(j=0;j<n-1;j++){
        if(p[j]<p[j+1]){
          sf=0;
          swp=p[j];
          p[j]=p[j+1];
          p[j+1]=swp;
        }
      }
      if(sf==1) break;
    }

    for(j=0,k=m-1;j<n;j++,k+=m) p[k]=0;

    for(j=0;j<n;j++) ans[i]+=p[j];
  }

  num=i;

  for(i=0;i<num;i++) printf("%d\n",ans[i]);

  return 0;
}