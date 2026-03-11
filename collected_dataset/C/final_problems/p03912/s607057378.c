#include<stdio.h>
int main()
{
	int n, m;
	scanf("%d %d", &n, &m);
	int i;
	int x[100005];
	for (i = 0; i < n; i++)
		scanf("%d", &x[i]);
	int a[100005];
	for (i = 0; i < 100005; i++)
		a[i] = 0;
	for (i = 0; i < n; i++)
		a[x[i]]++;
	int b[100005];
	for (i = 0; i < m; i++)
		b[i] = 0;
	for (i = 0; i < n; i++)
		b[x[i] % m]++;
	int ans = b[0] / 2;
	for (i = 1; 2 * i < m; i++)
	{
		if (b[i] > b[m - i])
		{
			ans += b[m - i];
			b[i] -= b[m - i];
			b[m - i] = 0;
		}
		else
		{
			ans += b[i];
			b[m - i] -= b[i];
			b[i] = 0;
		}
	}
	if (m % 2 == 0)
	{
		ans += b[m / 2] / 2;
		b[m / 2] = 0;
	}
	for (i = 0; i < 100005; i++)
	{
		if (b[i % m] > 1)
		{
			if (a[i] > 1)
			{
				ans++;
				a[i] -= 2;
				b[i % m] -= 2;
				i--;
			}
		}
	}
	printf("%d\n", ans);
	return 0;
}