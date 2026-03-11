#include<stdio.h>
int main()
{
  int a,b,i,j;
  while(1)
	   {
  scanf("%d%d",&a,&b);
  if(a==0&&b==0){break;}

  for(i=1;i<=a;i++)
    {
      for(j=1;j<=b;j++)
	{
	  if(j%2==1)
	    {
	      if(i%2==1)printf("#");
	      else printf(".");
	    }
	  else 
	    {
              if(i%2==1)printf(".");
	      else printf("#");
	    }
	}
      printf("\n");
    }
  printf("\n");
    
	   }
  return 0;
}

