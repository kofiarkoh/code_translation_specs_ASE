// Aizu Vol-1 0153: Triangle and Circle
// 2017.8.12 bal4u@uu

#include <stdio.h>

long long x[10], y[10];
long long xe, ye, r;

int atSameSide(long long x1, long long y1, long long x2, long long y2,
	long long xa, long long ya, long long xb, long long yb)
{
	long long sa, sb;
	sa = (x2 - x1) * (ya - y1) + (y2 - y1) * (x1 - xa);
	sb = (x2 - x1) * (yb - y1) + (y2 - y1) * (x1 - xb);
	return (sa >= 0 && sb >= 0) || (sa < 0 && sb < 0);
}

int between(long long x1, long long y1, long long x2, long long y2,
	long long x, long long y)
{
	long long sa, sb;
	sa = (x2 - x1) * (x - x1) + (y2 - y1) * (y - y1);
	sb = (x1 - x2) * (x - x2) + (y1 - y2) * (y - y2);
	return (sa >= 0 && sb >= 0) || (sa < 0 && sb < 0);
}

int disCmp(long long x1, long long y1, long long x2, long long y2,
	long long x, long long y, long long r)
{
	long long a, b, c, t;			/* ax + by + c = 0 */
	long long squ;
	a = y1 - y2, b = x2 - x1, c = y2*x1-x2*y1;
	squ = a*a + b*b;
	if (squ == 0) return 0;
	t = (a * x) + (b * y) + c;
	t = t*t - r*r*squ;
	if (t > 0) return 1;
	if (t < 0) return -1;
	return 0;
}

int main()
{
	int i, f;
	long long d[3], rr;

	while (1) {
		for (i = 0; i < 3; i++) {
			scanf("%lld%lld", x + i, y + i);
			if (!i && !x[0] && !y[0]) return 0;
			x[i + 3] = x[i], y[i + 3] = y[i];
		}
		scanf("%lld%lld%lld", &xe, &ye, &r); rr = r*r;

		for (i = 0; i < 3; i++)
			d[i] = (x[i] - xe)*(x[i] - xe) + (y[i] - ye)*(y[i] - ye);
		if (rr >= d[0] && rr >= d[1] && rr >= d[2]) { puts("b"); continue; }
		if (rr >= d[0] || rr >= d[1] || rr >= d[2]) { puts("c"); continue; }

		for (f = 1, i = 0; f && i < 3; i++)
			f = atSameSide(x[i], y[i], x[i + 1], y[i + 1],
				x[i + 2], y[i + 2], xe, ye);
		for (i = 0; i < 3; i++)
			d[i] = disCmp(x[i], y[i], x[i + 1], y[i + 1], xe, ye, r);
		if (f) { puts(d[0] >= 0 && d[1] >= 0 && d[2] >= 0 ? "a" : "c"); continue; }
		if (d[0] > 0 && d[1] > 0 && d[2] > 0) { puts("d"); continue; }

		for (f = 1, i = 0; f && i < 3; i++)
			if (between(x[i], y[i], x[i + 1], y[i + 1], xe, ye) && d[i] <= 0) f = 0;
		puts(f ? "d" : "c");
	}
	return 0;
}