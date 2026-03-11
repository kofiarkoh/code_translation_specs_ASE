#include <stdio.h>

int		main()
{
	double	A, B, K;
	int		r;

	A = 0;
	B = 0;
	K = 0;
	r = scanf("%lf %lf %lf\n", &A, &B, &K);
	if (r != 3)
		return (0);
	A = A - K;
	if (A < 0)
	{
		B += A;
		A = 0;
	}
	printf("%.0f %.0f\n", A, B);
	return (0);
}
