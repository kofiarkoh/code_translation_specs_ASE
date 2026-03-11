#include<stdio.h>
int main()
{
  int n,i,j,h=0,w=0,a[100000],b[100000],c[100000],z=0;
  int a1,a2,b1,b2,b3,c1,c2;
  scanf("%d",&n);
 for(i=0;i<n;i++)
 {
    scanf("%d",&b[i]);
 }
 for(i=0;i<n;i++)
 {
    scanf("%d",&a[i]);
 }
 for(i=0;i<n;i++)
 {
    scanf("%d",&c[i]);
 }
  for(i=0;i<n;i++)
  {
      h=0;
      w=0;
    for(j=0;j<n;j++)
    {
      if(a[i]>b[j])
      {
      h++;
      }
    }
    for(j=0;j<n;j++)
    {
      if(a[i]<c[j])
      {
      w++;
      }
    }
    z=z+(h*w);
  }
  printf("%d",z);
  return 0;
}
