#include <stdio.h>
#include <string.h>

int main()
{
  int N,K;
  
  scanf("%d %d",&N,&K);
  
  
  int a[100]={0};
  
  for(int i=0;i<K;i++)
  {
    int d;
    scanf("%d",&d);
    for(int j=0;j<d;j++)
    {
      int tmp;
      scanf("%d",&tmp);
      a[tmp-1]++;
    }
  }
  
  int ans=0;
  for(int i=0;i<N;i++)
  {
    if(a[i]==0)
      ans++;
  }
  printf("%d",ans);
}