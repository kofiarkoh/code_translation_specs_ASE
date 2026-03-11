#include <stdio.h>
#include <math.h>

double cal(double q, double x);

int main(void) {
	double q;
	while (1) {
		scanf("%lf", &q);
		if (q == -1.0)
			break;
		printf("%lf\n", cal(q, q/2.0));
	}

	return 0;
}

double cal(double q, double x) {
	x = x - (x*x*x-q)/(3*x*x);
	if (fabs(x*x*x-q) >= 0.00001*q)
		x = cal(q, x);
	return x;
}