#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(void)
{
	int N, T, A;
	scanf("%d%d%d",&N,&T,&A);
	int i = 0;
	int h[N];
	double t[N];
	double tn[N];
	while (i < N)
	{
		scanf("%d", &h[i]);
		tn[i] = h[i] * 0.006;
		t[i] = fabs((T - tn[i]) - A);
		i++;
	}
	int ans = 0;
	int min = t[0];
	i = 0;
	while (i < N)
	{
		// printf("%f ", t[i]);
		if (t[i] < min)
		{ 
			min = t[i];
			ans = i;
		}
		i++;
	}
	printf("%d\n", ans+1);
	return 0;
}