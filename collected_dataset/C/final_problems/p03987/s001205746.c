#include<stdio.h>

int main()
{
	int n, i, j, arr[200000], min;
	long sum= 0;

	scanf("%d", &n);

	for(i= 0; i< n; i++)
		scanf("%d", &arr[i]);

	for(i= 0; i< n; i++)
	{
		sum+= (min= arr[i]);

		for(j= i+ 1; j< n; j++)
		{
			if(arr[j]< min)
				sum+= (min= arr[j]);
			else
				sum+= min;
		}
	}

	printf("%ld\n", sum);

	return 0;
}