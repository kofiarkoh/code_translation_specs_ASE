#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define mem(x,y)		(x*)malloc(sizeof(x)*(y))

int main(void)
{
	int n;
	while(scanf("%d",&n),n)
	{
		int i,j;
		int* table = mem(int,n);
		for(i = 0;i < n;i++)
		{
			scanf("%d",&table[i]);
			if(i)
				table[i] += table[i-1];
		}
		int max = -1000000000;
		for(i = 0;i < n;i++)
		{
			if(table[i] > max)
				max = table[i];
			for (j = 0; j < i; j += 1)
			{
				if((table[i]-table[j]) > max)
					max = table[i]-table[j];
			}
		}
		printf("%d\n",max);
	}
	return 0;
}