#include <stdio.h>

int main()
{
  int num = 0,n;
  int a[200000];

  scanf("%d",&n);

  int i;

  for(i=0;i<n;i++)
    scanf("%d",&a[i]);


  int j,k,x;

      for(j=1;j<n;j++)
      	{
	  x =a[j];
	  if(a[j-1]<x)
	    {
	      k=j;
	      do{
		a[k] = a[k-1];
		k--;
	      }while(k>0&&a[k] < x);
	      a[k] = x;
	    }
	}
	
      
      for(j=0;j<n-1;j++)
	{
	  while(a[j] == -1)
	    j++;
     
	for(k=j+1;k<n;k++)
	  {
	    if(a[k]!=-1&&a[k]%2==a[j]%2)
	      {
		x = a[k] + a[j];

		while(x%2 != 1)
		  {
		    x /= 2;
		  };

		if(x == 1)
		  {
		    num++;
		    a[k] = -1;
		    break;
		  }
	      }
	  }
	}

  printf("%d\n",num);
  return 0;
}

