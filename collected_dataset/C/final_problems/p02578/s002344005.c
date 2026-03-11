#include<stdio.h>
int main()
{
  int N;
  int A[200010]={};
  int max;
  int sum=0;
  scanf("%d",&N);
  for(int i=0;i<N;i++) scanf("%d",&A[i]);
  max=A[0];
  for(int j=1;j<N;j++)
  {
    if(A[j]>=max)
    {
      max=A[j];
    }
    else 
    {
      sum=sum+max-A[j];
    }
  }
  printf("%d",sum);
  return 0;
}
