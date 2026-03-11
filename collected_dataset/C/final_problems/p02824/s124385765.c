#include<stdio.h>
#include<stdlib.h>
int asc(const void* a, const void* b)
{
	if (*(int*)a > * (int*)b)return -1;
	if (*(int*)a < *(int*)b)return 1;
	return 0;
}
int main()
{
	int n, m, v, p;
	scanf("%d %d %d %d", &n, &m, &v, &p);
	int i;
	int a[100005];
	for (i = 0; i < n; i++)
		scanf("%d", &a[i]);
	qsort(a, n, sizeof(int), asc);
	int min, mid, max;
	int count;
	min = 0;
	max = n;
	while (max - min > 1)
	{
		mid = (max + min) / 2;
		if (mid < p)
		{
			min = mid;
			continue;
		}
		if (a[mid] + m < a[p - 1])
		{
			max = mid;
			continue;
		}
		count = 0;
		for (i = 0; i < n; i++)
		{
			if (i >= mid)
				count += m;
			else if (i < p - 1)
				count += m;
			else
				count += a[mid] + m - a[i];
		}
		if (count >= m * v)
			min = mid;
		else
			max = mid;
	}
	printf("%d\n", max);
	return 0;
}