// Aizu Vol-1 0146: Lupin The 4th
// 2017.9.3 bal4u@uu

#include <stdio.h>

#define INF 1.0e20
#define abs(a) ((a)>=0?(a):-(a))
typedef struct { int s, d, v; } T;
T ku[17]; int n;
double ans;
long long ord;

double combi(double t, int v, int p, int last, int m, long long o)
{
	int i, next, b, v2, p2;
	double min, t2, x, y;
	long long o2;

	if (m == n) {
		if (t < ans) ans = t, ord = o;
		return t;
	}

	for (min = INF, next = -1, b = 1, i = 0; i < n; i++, b <<= 1) {
		if (p & b) continue;
		x = t + abs(ku[last].d - ku[i].d) * (7 + 2*v) / 200.0;
		y = combi(x, v + ku[i].v, p | b, i, m+1, (o << 4) | i);
		if (y < min) next = i, min = y, t2 = x, v2 = v + ku[i].v, p2 = p | b, o2 = (o << 4) | i;
	}
	if (next >= 0) return combi(t2, v2, p2, next, m+1, o2);
	return min;
}

int main()
{
	int i;

	scanf("%d", &n);
	for (i = 0; i < n; i++) scanf("%d%d%d", &ku[i].s, &ku[i].d, &ku[i].v);
	for (ans = INF, i = 0; i < n; i++)	combi(0, ku[i].v, 1 << i, i, 1, i);
	for (i = 0; i < n; i++) {
		if (i > 0) putchar(' ');
		printf("%d", ku[(int)(ord >> (n-i-1)*4) & 0xf].s);
	}
	putchar('\n');
	return 0;
}