#include<stdio.h>
int main()
{
  int num[6];
  int top=0,front=1,right=2,left=3,back=4,bottom=5;
  int i,j,n;
  int x,y;
  int rep;
  int aftertop,afterfront;
  for(i=0;i<6;i++) scanf("%d",&num[i]);
  scanf("%d",&n);
  for(i=0;i<n;i++)
    {
      scanf("%d%d",&x,&y);
      for(j=0;j<6;j++)
    {
      if(x==num[j])
        {
          aftertop=j;
          break;
        }
    }
      for(j=0;j<6;j++)
    {
      if(y==num[j])
        {
          afterfront=j;
          break;
        }
    }
      top=0;
      front=1;
      right=2;
      left=3;
      back=4;
      bottom=5;
      while((top!=aftertop)&&(front!=afterfront))
    {
      rep=top;
      top=front;
      front=bottom;
      bottom=back;
      back=rep;
    }
      while(top!=aftertop)
    {
      rep=top;
      top=right;
      right=bottom;
      bottom=left;
      left=rep;
    }
      while(front!=afterfront)
    {
      rep=front;
      front=right;
      right=back;
      back=left;
      left=rep;
    }
      printf("%d\n",num[right]);
    }
  return 0;
}
