#include<stdio.h>
int n;
int a[200005];
int u[400005], v[400005];
int h[400005], l;
int c[400005];
int comp_h(int x, int y)
{
	if (u[h[x]] > u[h[y]])
		return 1;
	else
		return -1;
}
void swap_h(int x, int y)
{
	int f = h[x];
	h[x] = h[y];
	h[y] = f;
	return;
}
void f1(int ne)
{
	h[l] = ne;
	int p = l;
	l++;
	while (p > 0)
	{
		if (comp_h((p - 1) / 2, p) > 0)
			swap_h((p - 1) / 2, p);
		p = (p - 1) / 2;
	}
	return;
}
int f2()
{
	l--;
	swap_h(0, l);
	int p = 0;
	for (;;)
	{
		if (2 * p + 2 < l)
		{
			if (comp_h(2 * p + 1, 2 * p + 2) > 0)
			{
				if (comp_h(p, 2 * p + 2) > 0)
					swap_h(p, 2 * p + 2);
				p = 2 * p + 2;
			}
			else
			{
				if (comp_h(p, 2 * p + 1) > 0)
					swap_h(p, 2 * p + 1);
				p = 2 * p + 1;
			}
		}
		else if (2 * p + 1 < l)
		{
			if (comp_h(p, 2 * p + 1) > 0)
				swap_h(p, 2 * p + 1);
			p = 2 * p + 1;
		}
		else
			break;
	}
	return h[l];
}
int ans[200005];
void dfs(int dp[], int dl, int now)
{
	int min, mid, max;
	min = -1;
	max = dl;
	while (max - min > 1)
	{
		mid = (max + min) / 2;
		if (dp[mid] >= a[now])
			max = mid;
		else
			min = mid;
	}
	dp[max] = a[now];
	if (max == dl)
		dl++;
	ans[now] = dl;
	min = -1;
	max = 2 * n - 2;
	while (max - min > 1)
	{
		mid = (max + min) / 2;
		if (u[c[mid]] < now)
			min = mid;
		else
			max = mid;
	}
	if (max < 2 * n - 2)
	{
		while (u[c[max]] == now)
		{
			if (ans[v[c[max]]] < 0)
				dfs(dp, dl, v[c[max]]);
			max++;
			if (max == 2 * n - 2)
				break;
		}
	}
}
int main()
{
	scanf("%d", &n);
	int i;
	for (i = 0; i < n; i++)
		scanf("%d", &a[i]);
	for (i = 0; i < n - 1; i++)
	{
		i *= 2;
		scanf("%d %d", &u[i], &v[i]);
		u[i]--;
		v[i]--;
		u[i + 1] = v[i];
		v[i + 1] = u[i];
		i /= 2;
	}
	l = 0;
	for (i = 0; i < 2 * n - 2; i++)
		f1(i);
	for (i = 0; i < 2 * n - 2; i++)
		c[i] = f2();
	for (i = 0; i < n; i++)
		ans[i] = -1;
	int dp[200005];
	dfs(dp, 0, 0);
	for (i = 0; i < n; i++)
		printf("%d\n", ans[i]);
	return 0;
}