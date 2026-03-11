#include<stdio.h>
#define N 10010
#define M 1000

int main(void){
  int i,num,j;
  int n,p,d1,d2;
  int sumd[N],maxd[M],ansp[M];

  for(i=0;;i++){
    scanf("%d",&n);
    if(n==0)
      break;
    for(j=0;j<N;j++){
      sumd[j]=0;
    }
    for(j=0;j<n;j++){
      scanf("%d%d%d",&p,&d1,&d2);
      sumd[p]+=(d1+d2);
    }
    maxd[i]=0;
    for(j=0;j<N;j++){
      if(maxd[i]<sumd[j]){
        maxd[i]=sumd[j];
        ansp[i]=j;
      }
    }
  }
  num=i;
  for(i=0;i<num;i++){
    printf("%d %d\n",ansp[i],maxd[i]);
  }
  return 0;
}