#include<stdio.h>
#include<math.h>
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
	int x, j, l;
	int count;
	int b[502];
	int p, q;
	for (i = 1; i <= sqrt(sum); i++)
	{
		if (sum % i == 0)
		{
			x = sum / i;
			for (j = 0; j < n; j++)
				b[j] = a[j] % x;
			for (j = 0; j < n - 1; j++)
			{
				if (b[j] > b[j + 1])
				{
					b[j] ^= b[j + 1];
					b[j + 1] ^= b[j];
					b[j] ^= b[j + 1];
					if (j > 0)
						j -= 2;
				}
			}
			count = 0;
			l = n - 1;
			for (j = 0; j < n; j++)
			{
				if (b[j] % x != 0)
				{
					count += b[j];
					b[j] %= x;
					while (b[j] > 0)
					{
						if (b[l] + b[j] < x)
						{
							b[l] += b[j];
							b[j] = 0;
						}
						else
						{
							b[j] -= x - b[l];
							b[l] = x;
							l--;
						}
					}
				}
			}
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
			for (j = 0; j < n - 1; j++)
			{
				if (b[j] > b[j + 1])
				{
					b[j] ^= b[j + 1];
					b[j + 1] ^= b[j];
					b[j] ^= b[j + 1];
					if (j > 0)
						j -= 2;
				}
			}
			count = 0;
			l = n - 1;
			for (j = 0; j < n; j++)
			{
				if (b[j] % x != 0)
				{
					count += b[j];
					b[j] %= x;
					while (b[j] > 0)
					{
						if (b[l] + b[j] < x)
						{
							b[l] += b[j];
							b[j] = 0;
						}
						else
						{
							b[j] -= x - b[l];
							b[l] = x;
							l--;
						}
					}
				}
			}
			if (count <= k)
			{
				printf("%d\n", x);
				return 0;
			}
		}
	}
}