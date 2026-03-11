#include <stdio.h>
#include <math.h>

typedef struct { double x, y; } point;
int input_point(point *p){ return scanf("%lf,%lf", &p->x, &p->y); }
double len(point *a, point *b) { return hypot(a->x - b->x, a->y - b->y); }
double area(point *x, point *y, point *z){
	double a = len(x, y), b = len(y, z), c = len(z, x);
	double s = (a + b + c) / 2;
	return sqrt(s * (s - a) * (s - b) * (s - c));
}

int main(int argc, char const *argv[])
{
	point v[36];
	double S;
	int n, i;

	n = 0;
	while (~input_point(v + n++));
	
	S = 0;
	for (i = 2; i < n; i++){
		S += area(v, v + i - 1, v + i);
	}
	printf("%f\n", S);

	return (0);
}