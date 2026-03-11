#include <stdio.h>
int main()
{
  int N,i;
  int a[200005],b[200005];
  scanf("%d",&N);
  for(i=0;i<N;i++)
    scanf("%d %d",&a[i],&b[i]);
if(N<1000)
  {if(N%2==0)
    printf("Second\n");
  else
    printf("First\n");
}
else
{
if(N%2==1)
    printf("Second\n");
  else
    printf("First\n");
}
  return 0;
}