#include<stdio.h>
int main()
{
	int n;
	scanf("%d", &n);
	int i;
	int x[55], y[55];
	for (i = 0; i < n; i++)
		scanf("%d %d", &x[i], &y[i]);
	int ans = 2000000009;
	int p, q;
	int j, k, l, a;
	for (i = 0; i < n; i++)
	{
		for (j = 0; j < n; j++)
		{
			if (i != j)
			{
				p = x[i] - x[j];
				q = y[i] - y[j];
				a = n;
				for (k = 0; k < n; k++)
					for (l = 0; l < n; l++)
						if (x[k] - p == x[l] && y[k] - q == y[l])
							a--;
				if (ans > a)
					ans = a;
			}
		}
	}
	printf("%d\n", ans);
	return 0;
}