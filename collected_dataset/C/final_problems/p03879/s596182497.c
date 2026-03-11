#include <stdio.h>
#include <math.h>

double max(double a, double b){
	return a >= b ? a : b;
}

int main(){
	double x1, y1, x2, y2, x3, y3, A, B, C, K, R = 0;
	scanf("%lf%lf%lf%lf%lf%lf", &x1, &y1, &x2, &y2, &x3, &y3);
	A = hypot(x1 - x2, y1 - y2);
	B = hypot(x2 - x3, y2 - y3);
	C = hypot(x3 - x1, y3 - y1);
//	printf("(A, B, C) = (%lf, %lf, %lf)\n", A, B, C);
	K = sqrt((A + B + C) / ((A + B - C) * (B + C - A) * (C + A - B)));
//	printf("K = %lf\n", K);
	R = max(R, A / (2 * (1 + A * K)));
	R = max(R, B / (2 * (1 + B * K)));
	R = max(R, C / (2 * (1 + C * K)));
	printf("%.12lf\n", R);
	return 0;
}