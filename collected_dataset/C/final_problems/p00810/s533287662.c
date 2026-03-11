// AOJ 1231: Super Star
// 2017.12.18 bal4u@uu

#include <stdio.h>
#include <string.h>
#include <math.h>

#define EPS 1e-8

typedef struct { double x, y, z; } PP;
PP p[31];

double dist(PP a, PP b)
{
	double x = a.x - b.x;
	double y = a.y - b.y;
	double z = a.z - b.z;
	return sqrt(x*x + y*y + z*z);
}

double solve_by_movement(int n)
{
	int  k, i, t;
	PP a;
	double move, max, d;

    a.x = a.y = a.z = 50;
	move = 0.5;
	while (move > EPS) {
		for (t = 0; t < 100; t++) {
			max = 0;
			for (i = 0; i < n; i++) {
				if ((d = dist(a, p[i])) > max) max = d, k = i;
			}
			a.x += (p[k].x - a.x)*move;
			a.y += (p[k].y - a.y)*move;
			a.z += (p[k].z - a.z)*move;
		}
		move /= 2;
	}
	return max;
}

int main()
{
	int n, i;

	while(scanf("%d", &n) && n > 0) {
		for (i = 0; i < n; i++) {
			scanf("%lf%lf%lf", &p[i].x, &p[i].y, &p[i].z);
		}
		printf("%.5lf\n", solve_by_movement(n));
	}
	return 0;
}