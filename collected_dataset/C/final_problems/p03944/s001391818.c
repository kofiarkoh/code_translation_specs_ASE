#include<stdio.h>

int main()
{
  int w_x,h_y;
  int n;
  int x[101]={},y[101]={},a[101]={};
  int i;
  int area;
  
  scanf("%d %d %d",&w_x,&h_y,&n);
  
   int left_x=0,below_y=0;
  int right_x=w_x,high_y=h_y;
  
  for(i=1;i<=n;i++)
  {
    scanf("%d %d %d",&x[i],&y[i],&a[i]);
  }
  
  for(i=1;i<=n;i++)
  { 
   if(a[i]==1)
   {
    if(left_x < x[i] )
    {
     left_x = x[i];
    }
   }
    else if(a[i]==2)
    {
      if(x[i] < right_x )
      {
        right_x = x[i];
      }
    }
    else if(a[i]==3)
    {
      if( below_y < y[i])
      {
        below_y = y[i];
      }
    }
    else if(a[i]==4)
    {
      if( y[i] < high_y  )
      {
        high_y = y[i];
      }
    }
  }
 
  if((left_x <= right_x)&&(below_y<=high_y))
  {
  area = (right_x - left_x) * (high_y - below_y);
  }
  else
  {
    area = 0;
  }
  printf("%d\n",area);
  
  return 0;
}