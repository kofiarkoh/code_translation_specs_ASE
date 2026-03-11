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
	long long int w, h;
	scanf("%lld %lld", &w, &h);
	long long int i, j;
	long long int p[100005], q[100005];
	for (i = 0; i < w; i++)
		scanf("%lld", &p[i]);
	for (j = 0; j < h; j++)
		scanf("%lld", &q[j]);
	qsort(p, w, sizeof(long long int), asc);
	qsort(q, h, sizeof(long long int), asc);
	long long int ans = 0;
	i = j = 0;
	for (;;)
	{
		if (i == w)
		{
			ans += q[j];
			j++;
		}
		else if (j == h)
		{
			ans += p[i];
			i++;
		}
		else
		{
			if (q[j] < p[i])
			{
				ans += q[j] * (w - i + 1);
				j++;
			}
			else
			{
				ans += p[i] * (h - j + 1);
				i++;
			}
		}
		if (i == w && j == h)
			break;
	}
	printf("%lld\n", ans);
	return 0;
}