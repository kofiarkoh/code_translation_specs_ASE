#include<stdio.h>
int main()
{
	int i, n,m;
	int a = 0, b = 0;
	scanf("%d", &n);
	for (i = 0; i < n; i++)
	{
		scanf("%d", &m);
		if (m % 4 == 0)
		{
			b++;
		}
		else if (m % 2 == 0)
		{
			a++;
		}
	}
	if ((n  -1 - b) <=  b&&a==0)
	{
		printf("Yes\n");
	}
else if ((n  -a) <= 2 * b&&a!=0)
	{
		printf("Yes\n");
	}

	else 
	{
		printf("No\n");
	}
	return 0;
}