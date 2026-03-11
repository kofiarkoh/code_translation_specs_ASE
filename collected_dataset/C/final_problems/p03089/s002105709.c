#include <stdio.h>
#include <stdlib.h>

typedef unsigned char bool;
int main(){
  int n;
  scanf("%d",&n);
  int a[n+1],b[n+1],i;
  bool flag=1;
  for(i=1;i<=n;i++)
    {
      scanf("%d",&a[i]);
      if(a[i]>i)flag=0;
      b[i]=0;
    }
  if(flag==0)
    {
      printf("-1\n");
      return 0;
    }
  //  printf("1\n");
  int j,temp,count;
  for(i=n;i>0;i--)
    {
      count=0;
      for(j=i+1;j<=n;j++)
	{
	  if(a[i]>=a[j]) count++;
	}
      //      printf("%2d ",count);
      for(j=0;j<count;j++)
	{
	  if(b[a[i]+j]>a[i])
	    count++;
	  //	  printf("%d:%d:%d ",b[i+j],a[i],count);
	}
      j=a[i]+count;
      //      printf("%2d\n",j);
      while(b[j]!=0)j++;
      b[j]=a[i];
    }
  //  printf("\n",j);
  for(i=1;i<=n;i++)
    printf("%d\n",b[i]);
  return 0;
}
