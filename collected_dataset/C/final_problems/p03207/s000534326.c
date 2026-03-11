#include<stdio.h>
int main(void) {
  int N,P[10],p=0,ans=0,i;
  scanf("%d",&N);
  for(i=0;i<N;i++)scanf("%d",&P[i]);
  for(i=0;i<N;i++)if(P[p]<P[i])p=i;
  for(i=0;i<N;i++)ans+=P[i];
  ans-=P[p]/2;
  printf("%d",ans);
  return 0;
}