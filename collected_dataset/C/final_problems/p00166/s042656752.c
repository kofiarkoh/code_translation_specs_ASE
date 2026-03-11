// Aizu Vol-1 0166: Area of Polygon
// 2017.8.8

#include <stdio.h>
#include <math.h>

#define M	0.01745329251994329576923690768489
#define EPS 0.0000001

int main()
{
	int n, v;
	double s1, s2;
	int a1, a2;

	while (1) {
		scanf("%d", &n); if (n == 0) break;
		s1 = s2 = 0, a1 = a2 = 0;
		while (--n > 0) {
			scanf("%d", &v); 
			a1 += v;
			s1 += sin((double)v*M);
		}
		s1 += sin((360.0 - v)*M);

		scanf("%d", &n);
		while (--n > 0) {
			scanf("%d", &v);
			a2 += v;
			s2 += sin((double)v*M);
		}
		s2 += sin((360.0 - v)*M);

		if (fabs(s1-s2) <= EPS) putchar('0');
		else if (s1 >= s2) putchar('1');
		else               putchar('2');
		putchar('\n');
	}
	return 0;
}