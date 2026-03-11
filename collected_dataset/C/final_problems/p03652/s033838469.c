#include<stdio.h>
int main()
{
	int n, m;
	scanf("%d %d", &n, &m);
	int i, j;
	int a[302][302];
	for (i = 0; i < n; i++)
	{
		for (j = 0; j < m; j++)
		{
			scanf("%d", &a[i][j]);
			a[i][j]--;
		}
	}
	int count[302];
	for (i = 0; i < m; i++)
		count[i] = 0;
	int use[302];
	for (i = 0; i < m; i++)
		use[i] = 0;
	int ans = n;
	int l[302];
	for (i = 0; i < n; i++)
		l[i] = 0;
	int max;
	for (i = 0; i < n; i++)
		count[a[i][0]]++;
	for (j = 1; j < m; j++)
	{
		max = 0;
		for (i = 0; i < m; i++)
			if (count[i] > count[max])
				max = i;
		if (ans > count[max])
			ans = count[max];
		use[max] = 1;
		for (i = 0; i < m; i++)
			count[i] = 0;
		for (i = 0; i < n; i++)
		{
			while (l[i] < m && use[a[i][l[i]]]>0)
				l[i]++;
			if (l[i] == m)
			{
				j = m + 10;
				break;
			}
			count[a[i][l[i]]]++;
		}
	}
	printf("%d\n", ans);
	return 0;
}