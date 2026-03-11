#include<stdio.h>
#include<stdlib.h>
#include<math.h>
 
int main()
{
	int n;
	while(scanf("%d", &n)&&2<=n<=2*pow(10,5))
		break;
	int *a=NULL;
    a = (int *)malloc(sizeof(int)*n);
	int i = 0;
	while (scanf("%d", &a[i]) && i < n-1&&-pow(10,9)<=a[i]<=pow(10,9))
		i++;
	int sum = 0;
	for (int j = 0; j<n; j++)
	{
		int x = j,y=0;
		for (y = j+1; y < n; y++)
			if (a[x] > a[y]) x = y;
		if (j != x)
		{
			int t;
			t = a[j];
			a[j] = a[x];
			a[x] = t;
		}
		sum += a[j];
     }
	int k ;
	int sum_1 ;
	for (int x = 1 ; x < n; x++)
	{
		sum_1 = 0;
		int j;
		for ( j = 0; j < n - x; j++)
			 sum_1 += a[j]; 
		 k = sum - 2 * sum_1;
		  if (k > 0 && x == 1)
		  {
			  printf("%d", k);
			  break;
		  }
		  else if (k == 0)
		  {
			  printf("0");
			  break;
		  }
		  else if (k < 0)
			  continue;
         else 
		 {
			 if (k >=(-1)*(k-2*a[j]))
			 {
				 printf("%d",(-1)*(k - 2* a[j])); break;
			 }
			 else
			 {
				 printf("%d", k);
				 break;
			 }
		 }
	}
	 
	return 0;
}