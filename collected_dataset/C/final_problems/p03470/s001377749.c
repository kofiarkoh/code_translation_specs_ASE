#include<stdio.h>
int main()
{
 int N,i,j,sum,tmp;
  int a[100]={0};
  scanf("%d",&N);
  sum=N;
  for(i=0;i<N;i++)
  {
    scanf("%d",&a[i]);
  }
  for(i=0;i<N-1;i++)
  {
    for(j=0;j<N-1;j++)
    {
     if(a[j]<=a[j+1])
     {
       tmp=a[j];
       a[j]=a[j+1];
       a[j+1]=tmp;
     } 
    }
  }
  for(i=0;i<N-1;i++)
  {
  if(a[i]==a[i+1])
  {
    sum=sum-1;
  }
  }
  printf("%d",sum);
  return 0;
}