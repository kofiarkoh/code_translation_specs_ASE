#include <stdio.h>

int gcd(int a, int b) {
	int tmp;
	if (b > a) {
		tmp = a;
		a = b;
		b = tmp;
	}

	while (b) {
		tmp = a % b;
		a = b;
		b = tmp;
	}
	return a;
}

void ex_gcd(int a, int b, int* g, int* x, int* y) {
	int r0, r1, r2, x0, x1, x2, y0, y1, y2, q;
	r0 = a; r1 = b;
	x0 = 1; x1 = 0;
	y0 = 0; y1 = 1;
	while (r1 > 0) {
//		printf("x0 = %d, y0 = %d\n", x0, y0);
		r2 = r0 % r1;
		q = r0 / r1;
		x2 = x0 - q * x1;
		y2 = y0 - q * y1;
		r0 = r1; r1 = r2;
		x0 = x1; x1 = x2;
		y0 = y1; y1 = y2;
	}
	*g = r0;
	*x = x0;
	*y = y0;
	return;
}

int main(void) {
	int a, b, g;
	int x = 0, y = 0;
	scanf("%d %d", &a, &b);
	
//	g = gcd(a, b);
	ex_gcd(a, b, &g, &x, &y);
	printf("%d %d\n", x, y);
	return 0;
}