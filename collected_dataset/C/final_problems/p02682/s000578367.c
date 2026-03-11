#include<stdio.h>
int main(void)
{
  int A,B,C,K,m;
  scanf("%d %d %d %d",&A,&B,&C,&K);
  if(A>=K)
  {
    m=A;
  }
  if(A<K)
  {
    m=A;
    K=K-A;
  }
  K=K-B;
  if(C>=K)
  {
    m=m+K*-1;
  }
  printf("%d",m);
  return 0;
}