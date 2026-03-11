#include<stdio.h>
long long int k[200005], l[200005], r[200005];
long long int h[200005], hl;
int comp_h(long long int a, long long int b, int z)
{
	if (z == 0)
	{
		if (l[h[a]] > r[h[a]] && l[h[b]] > r[h[b]])
		{
			if (k[h[a]] > k[h[b]])
				return 1;
			return -1;
		}
		else if (l[h[a]] < r[h[a]] && l[h[b]] < r[h[b]])
		{
			if (k[h[a]] < k[h[b]])
				return 1;
			return -1;
		}
		else
		{
			if (l[h[a]] <= r[h[a]])
				return 1;
			return -1;
		}
	}
	else if (z == 1)
	{
		if (l[h[a]] - r[h[a]] > l[h[b]] - r[h[b]])
			return 1;
		return -1;
	}
	else
	{
		if (r[h[a]] - l[h[a]] > r[h[b]] - l[h[b]])
			return 1;
		return -1;
	}
}
void swap_h(long long int a, long long int b)
{
	long long int f = h[a];
	h[a] = h[b];
	h[b] = f;
	return;
}
void f1(long long int ne, int z)
{
	h[hl] = ne;
	long long int p = hl;
	hl++;
	while (p > 0)
	{
		if (comp_h((p - 1) / 2, p, z) > 0)
			swap_h((p - 1) / 2, p);
		p = (p - 1) / 2;
	}
	return;
}
long long int f2(int z)
{
	hl--;
	swap_h(0, hl);
	long long int p = 0;
	for (;;)
	{
		if (2 * p + 2 < hl)
		{
			if (comp_h(2 * p + 1, 2 * p + 2, z) > 0)
			{
				if (comp_h(p, 2 * p + 2, z) > 0)
					swap_h(p, 2 * p + 2);
				p = 2 * p + 2;
			}
			else
			{
				if (comp_h(p, 2 * p + 1, z) > 0)
					swap_h(p, 2 * p + 1);
				p = 2 * p + 1;
			}
		}
		else if (2 * p + 1 < hl)
		{
			if (comp_h(p, 2 * p + 1, z) > 0)
				swap_h(p, 2 * p + 1);
			p = 2 * p + 1;
		}
		else
			break;
	}
	return h[hl];
}
int main()
{
	int t;
	scanf("%d", &t);
	long long int n;
	long long int i, j;
	long long int c[200005];
	long long int ans;
	long long int p;
	for (; t > 0; t--)
	{
		scanf("%lld", &n);
		for (i = 0; i < n; i++)
			scanf("%lld %lld %lld", &k[i], &l[i], &r[i]);
		hl = 0;
		for (i = 0; i < n; i++)
			f1(i, 0);
		for (i = 0; i < n; i++)
			c[i] = f2(0);
		ans = 0;
		j = 0;
		while (j<n && l[c[j]]>r[c[j]])
			j++;
		hl = 0;
		for (i = 0; i < j; i++)
		{
			ans += l[c[i]];
			f1(c[i], 1);
			if (hl > k[c[i]])
			{
				p = f2(1);
				ans -= l[p];
				ans += r[p];
			}
		}
		while (j < n && l[c[j]] == r[c[j]])
		{
			ans += l[c[j]];
			j++;
		}
		hl = 0;
		for (i = j; i < n; i++)
		{
			ans += r[c[i]];
			f1(c[i], 2);
			if (hl > n - k[c[i]])
			{
				p = f2(2);
				ans -= r[p];
				ans += l[p];
			}
		}
		printf("%lld\n", ans);
	}
	return 0;
}