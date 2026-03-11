#include <stdio.h>

int main()
{
	double r, pi = 3.1415926535898;
	scanf("%lf", &r);
	printf("%.6lf %.6lf", r * r * pi, r * 2 * pi);

	return 0;
}

