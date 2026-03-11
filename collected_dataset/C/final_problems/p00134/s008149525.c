#include <stdio.h>

int main(void)
{

	int n, i;
	int p;
	double sum;
	int ave;

	while (scanf("%d", &n) != EOF)
	{
		if (n != 0)
		{
			sum = 0;
			for (i = 0; i < n; i++)
			{
				scanf("%d", &p);
				sum += p;
			}

			ave = sum / n;

			printf("%d\n", ave);
		}
	}
	
	return 0;
}