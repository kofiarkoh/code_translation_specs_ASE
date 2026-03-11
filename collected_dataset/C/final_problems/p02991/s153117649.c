#include<stdio.h>
#include<stdlib.h>
int u[100005], v[100005];
int asc(const void* a, const void* b)
{
	return u[*(int*)a] - u[*(int*)b];
}
int main()
{
	int n, m;
	scanf("%d %d", &n, &m);
	if (m < 2)
	{
		printf("-1\n");
		return 0;
	}
	int i, j;
	for (i = 0; i < m; i++)
	{
		scanf("%d %d", &u[i], &v[i]);
		u[i]--;
		v[i]--;
	}
	int s, t;
	scanf("%d %d", &s, &t);
	s--;
	t--;
	int c[100005];
	for (i = 0; i < m; i++)
		c[i] = i;
	qsort(c, m, sizeof(int), asc);
	int min, max, mid;
	int d[2][100005];
	for (i = 0; i < n; i++)
		d[0][i] = -1;
	d[0][s] = 0;
	for (i = 0; i < 404; i++)
	{
		for (j = 0; j < n; j++)
		{
			if (d[0][j] == i)
			{
				min = 0;
				max = m - 1;
				while (max - min > 1)
				{
					mid = (max + min) / 2;
					if (u[c[mid]] < j)
						min = mid;
					else
						max = mid;
				}
				if (u[c[min]] < j)
					min++;
				while (u[c[min]] == j)
				{
					d[1][v[c[min]]] = i + 1;
					min++;
					if (min == m)
						break;
				}
			}
		}
		for (j = 0; j < n; j++)
			if (d[1][j] == i + 1)
				d[0][j] = i + 1;
		if (d[0][t] % 3 == 0)
		{
			printf("%d\n", (i + 1) / 3);
			return 0;
		}
	}
	printf("-1\n");
	return 0;
}