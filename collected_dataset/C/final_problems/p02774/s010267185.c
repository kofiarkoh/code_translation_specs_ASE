#include<stdio.h>
#include<stdlib.h>
int asc(const void* a, const void* b)
{
	if (*(long long int*)a > * (long long int*)b)return 1;
	if (*(long long int*)a < *(long long int*)b)return -1;
	return 0;
}
int main()
{
	long long int n, k;
	scanf("%lld %lld", &n, &k);
	long long int i;
	long long int a[200005];
	for (i = 0; i < n; i++)
		scanf("%lld", &a[i]);
	qsort(a, n, sizeof(long long int), asc);
	long long int amin, amid, amax;
	amin = -1000000000000000018;
	amax = -1 * amin;
	long long int min, mid, max;
	long long int count;
	while (amax - amin > 1)
	{
		amid = (amax + amin) / 2;
		count = 0;
		for (i = 0; i < n; i++)
		{
			min = -1;
			max = n;
			while (max - min > 1)
			{
				mid = (max + min) / 2;
				if (a[i] >= 0)
				{
					if (a[mid] * a[i] <= amid)
						min = mid;
					else
						max = mid;
				}
				else
				{
					if (a[mid] * a[i] <= amid)
						max = mid;
					else
						min = mid;
				}
			}
			if (a[i] >= 0)
				count += max;
			else
				count += n - max;
			if (a[i] * a[i] <= amid)
				count--;
		}
		count /= 2;
		if (count < k)
			amin = amid;
		else
			amax = amid;
	}
	printf("%lld\n", amax);
	return 0;
}