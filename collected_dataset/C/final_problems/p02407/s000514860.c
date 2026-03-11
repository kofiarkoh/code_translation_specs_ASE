#include <stdio.h>

int main(void)
{
	int n = 0, i, num = 0;

	scanf("%d", &n);
	
	int a_array[n];

	for (i = 0; i < n; i++)
	{
		scanf("%d", &num);
		a_array[i] = num;
	}
	for (i = n - 1; i >= 0; i--)
	{
		if (i == 0)
		{
			printf("%d\n", a_array[i]);
		}
		else
		{
			printf("%d ", a_array[i]);
		}
	}

	return 0;
}