#include <stdio.h>

void swap(int *a, int *b)
{
	int tmp;
	tmp = *a;
	*a = *b;
	*b = tmp;
}

int abs_int(int a)
{
	if (a >= 0) return a;
	return -a;
}

void merge_sort (int array[], int left, int right)
{
	int i, j, k, mid;
	int work[300000];

	if (left < right)
	{
		mid = (left + right) / 2;
		merge_sort(array, left, mid);
		merge_sort(array, mid+1, right);
		for (i = mid; i >= left; i--)
		{
			work[i] = array[i];
		}
		for (j = mid+1; j <= right; j++)
		{
			work[right-(j-(mid+1))] = array[j];
		}
		i = left; j = right;
		for (k = left; k <= right; k++)
		{
			if (abs_int(work[i]) > abs_int(work[j]))
			{
				array[k] = work[i++];
			}
			else
			{
				array[k] = work[j--];
			}
		}
	}
}

int main(void)
{
	int n, k;
	int a[300000];
	long long int ans = 1;
	long long int ans2 = 1, ans3 = 1;
	int i, j, l, f, f2, f3;

	scanf("%d%d", &n, &k);

	f = 1;
	for (i = 0; i < n; i++)
	{
		scanf("%d", &a[i]);
		if (a[i] > 0) f = 0;
	}

	merge_sort(a, 0, n - 1);

	if (n == k)
	{
		for (i = 0; i < k; i++)
		{
			ans *= a[i] % 1000000007;
			ans %= 1000000007;
		}
	}
	else if (f == 1 && k % 2 == 1)
	{
		for (i = 0; i < k; i++)
		{
			ans *= a[n - 1 - i] % 1000000007;
			ans %= 1000000007;
		}
	}
	else
	{
		for (i = 0; i < k; i++)
		{
			ans *= a[i] % 1000000007;
			ans %= 1000000007;
		}
		if (ans < 0)
		{
			f2 = f3 = 0;
			for (i = k - 1; i >= 0; i++)
			{
				if (a[i] < 0 && f2 == 0)
				{
					f2 = 1;
					for (j = k; j > n; j++)
					{
						if (a[j] > 0)
						{
							for (l = 0; l < k; l++)
							{
								if (l == i) continue;
								ans2 *= a[l];
								ans2 %= 1000000007;
							}
							ans2 *= a[j];
							break;
						}
					}
				}
				else if(f3 == 0)
				{
					f3 = 1;
					for (j = k; j > n; j++)
					{
						if (a[j] < 0)
						{
							for (l = 0; l < k; l++)
							{
								if (l == i) continue;
								ans3 *= a[l];
								ans3 %= 1000000007;
							}
							ans3 *= a[j] % 1000000007;
							break;
						}
					}
				}
				if (ans2 > ans3) ans = ans2;
				else ans = ans3;
			}
		}
	}

	ans %= 1000000007;
	if (ans < 0) ans += 1000000007;

	printf("%lld\n", ans);

	return 0;
}
