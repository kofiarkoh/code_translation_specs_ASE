#include<stdio.h>
#define SIZE 200000
int det(int);
int main()
{
  int n;
  int a[SIZE];
  int i,sum=0,dist,ans;
  scanf("%d",&n);
  for(i=0;i<n;i++)
    {
      scanf("%d",&a[i]);
      sum+=a[i];
    }
  dist=ans=sum-2*a[0];
  ans=det(ans);
  for(i=1;i<n-1;i++)
    {
      dist-=2*a[i];
      if(det(dist)<ans) ans=det(dist);
    }
  printf("%d\n",ans);
  return 0;
}
int det(int x)
{
  if(x<0) return x*-1;
  else return x;
}

