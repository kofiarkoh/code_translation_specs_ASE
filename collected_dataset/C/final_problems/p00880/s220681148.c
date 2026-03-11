#include <stdio.h>
#include <math.h>
#include <stdlib.h>

double X1, X2, X3, Y1, Y2, Y3; //coor
double r1, r2, r3;             //radii
double e1, e2, e3;             //edge
double a1, a2, a3;             //angle
double r;

void calc()
{
	e1 = sqrt(pow(X2 - X3, 2) + pow(Y2 - Y3, 2));
	e2 = sqrt(pow(X1 - X3, 2) + pow(Y1 - Y3, 2));
	e3 = sqrt(pow(X1 - X2, 2) + pow(Y1 - Y2, 2));
	a1 = acos((e2 * e2 + e3 * e3 - e1 * e1) / (2 * e2 * e3));
	a2 = acos((e3 * e3 + e1 * e1 - e2 * e2) / (2 * e1 * e3));
	a3 = acos((e1 * e1 + e2 * e2 - e3 * e3) / (2 * e1 * e2));
	r = e2 * e3 * sin(a1) / (e1 + e2 + e3);
	r1 = r * ((1 + tan(a2 / 4)) * (1 + tan(a3 / 4))) / (2 * (1 + tan(a1 / 4)));
	r2 = r * ((1 + tan(a1 / 4)) * (1 + tan(a3 / 4))) / (2 * (1 + tan(a2 / 4)));
	r3 = r * ((1 + tan(a1 / 4)) * (1 + tan(a2 / 4))) / (2 * (1 + tan(a3 / 4)));
}

int main()
{
	while (1)
	{
		scanf("%lf %lf %lf %lf %lf %lf", &X1, &Y1, &X2, &Y2, &X3, &Y3);
		if (X1 == 0 && X2 == 0 && X3 == 0 && Y1 == 0 && Y2 == 0 && Y3 == 0)
			break;
		calc();
		printf("%lf %lf %lf\n", r1, r2, r3);
	}
}
