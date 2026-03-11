#include <stdio.h>
void	ft_swap(long *a, long *b)
{
	long tmp;
	
	tmp = *a;
	*a = *b;
	*b = tmp;
}

/* 昇順 */
void	a_qsort(long *tab, long left, long right)
{
	long Left = left;
	long Right = right;
	long pivot = tab[(left + right) / 2];

	while (1)
	{
		while (tab[Left] < pivot)
			Left++;
		while (tab[Right] > pivot)
			Right--;
		if (Left >= Right)
			break ;
		ft_swap(&tab[Left], &tab[Right]);
		Left++;
		Right--;
	}

	if (Left - 1 > left)
		a_qsort(tab, left, Left - 1);
	if (Right + 1 < right)
		a_qsort(tab, Right + 1, right);
}

int		main()
{
	long A,B,M;
	scanf("%ld%ld%ld", &A,&B,&M);
	long a[A + 1],b[B + 1];
	long m[M + 1][4];
	long dis[M + 1];
	long i,j;
	for (i = 1; i <= A; i++)
		scanf("%ld", &a[i]);
	for (i = 1; i <= B; i++)
		scanf("%ld", &b[i]);
	for (i = 1; i <= M; i++)
	{
		for (j = 1;j <= 3; j++)
		{
			scanf("%ld", &m[i][j]);
		}
	}
	for (i = 1; i <= M; i++)
	{
		dis[i] = (a[m[i][1]] + b[m[i][2]]) - m[i][3];
	}
	a_qsort(a, 1, A);
	a_qsort(b, 1, B);
	long min = a[1] + b[1];
	for (i = 1; i <= M; i++)
	{
		if (dis[i] < min)
			min = dis[i];
	}
	printf("%ld\n", min);
	return (0);
}