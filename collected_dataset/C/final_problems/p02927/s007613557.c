#include<stdio.h>
int main()
{
  int m,d,c=0,prod,diff,diff1,k=22;
  scanf("%d%d",&m,&d);
  if(d>=22)
  {
    while(k<=d)
    {
      if((k%10)>=2)
      {
        diff=k%10;
        diff1=k/10;
        prod=diff*diff1;
        if(prod<=m)
        {
          c++;
        }
      }
      k++;
    }
  }
  printf("%d\n",c);
  return 0;
}