#include<stdio.h>
#include<math.h>
#include<stdlib.h>
int asc(const void* a, const void* b)
{
	if (*(int*)a > * (int*)b)return 1;
	if (*(int*)a < *(int*)b)return -1;
	return 0;
}
int myabs(int n)
{
	if (n < 0)
		n *= -1;
	return n;
}
int main()
{
	int n, k;
	scanf("%d %d", &n, &k);
	int i;
	int a[502];
	for (i = 0; i < n; i++)
		scanf("%d", &a[i]);
	int sum = 0;
	for (i = 0; i < n; i++)
		sum += a[i];
	int c, x, j;
	int count;
	int b[502];
	for (i = 1; i <= sqrt(sum); i++)
	{
		if (sum % i == 0)
		{
			x = sum / i;
			for (j = 0; j < n; j++)
				b[j] = a[j] % x;
			qsort(b, n, sizeof(int), asc);
			c = 0;
			count = 0;
			for (j = 0; j < n - 1; j++)
			{
				if (myabs(c + b[j]) < myabs(c - x + b[j]))
				{
					c += b[j];
					count += b[j];
				}
				else
				{
					c -= x - b[j];
				}
			}
			if (c < 0)
				count -= c;
			if (count <= k)
			{
				printf("%d\n", x);
				return 0;
			}
		}
	}
	for (i = (int)sqrt(sum); i > 0; i--)
	{
		if (sum % i == 0)
		{
			x = i;
			for (j = 0; j < n; j++)
				b[j] = a[j] % x;
			qsort(b, n, sizeof(int), asc);
			c = 0;
			count = 0;
			for (j = 0; j < n - 1; j++)
			{
				if (myabs(c + b[j]) < myabs(c - x + b[j]))
				{
					c += b[j];
					count += b[j];
				}
				else
				{
					c -= x - b[j];
				}
			}
			if (c < 0)
				count -= c;
			if (count <= k)
			{
				printf("%d\n", x);
				return 0;
			}
		}
	}
}