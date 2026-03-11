#include<stdio.h>
int main()
{
	int n;
	scanf("%d", &n);
	int i, j, k;
	char s[202][202];
	for (i = 0; i < n; i++)
		scanf("%s", s[i]);
	int ans = -1, v;
	int p;
	for (k = 0; k < n; k++)
	{
		int g[202];
		for (i = 0; i < n; i++)
			g[i] = -1;
		g[k] = 1;
		int q[10006], l = 0, r = 0;
		r++;
		q[0] = k;
			int m;
		while (l < r)
		{
			m = q[l];
			l++;
			for (i = 0; i < n; i++)
			{
				if (s[m][i] == '1' && g[i] < 0)
				{
					g[i] = g[m] + 1;
					q[r] = i;
					r++;
				}
			}
		}
		p = 0;
		for (i = 0; i < n; i++)
			for (j = 0; j < n; j++)
				if (s[i][j] == '1')
					if (g[i] - g[j] != 1 && g[j] - g[i] != 1)
						p++;
		if (p == 0)
		{
			v = 0;
			for (i = 0; i < n; i++)
				if (v < g[i])
					v = g[i];
			if (ans < v)
				ans = v;
		}
	}
	printf("%d\n", ans);
	return 0;
}