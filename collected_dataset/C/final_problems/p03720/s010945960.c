#include <stdio.h>

int main()
{
	int N, M;
	scanf("%d%d", &N,&M);
	int a[M + 1],b[M + 1];
	int n[N + 1];
	int i = 0;
	while (i < M)
	{
		scanf("%d",&a[i]);
		scanf("%d", &b[i]);
		i++;
	}
	int j;
	i = 1;
	while (i <= N)
	{
		j = 0;
		n[i] = 0;
		while (j < M)
		{
			if (i == a[j] || i == b[j])
				n[i] += 1;
			j++;
		}
		printf("%d\n", n[i]);
		i++;
	}
	return (0);
}
