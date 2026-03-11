// AOJ 2139: Memory Match
// 2017.11.13

#include <stdio.h>
#include <string.h>

double m[1001][1001];
char f[1001][1001];

double calc(int unopen, int known)
{
	double u, k, r;
	
	if (unopen == known) { f[unopen][known] = 1; return 0; }
	if (f[unopen][known]) return m[unopen][known];

	u = unopen, k = known, r = 0;
	if (known > 0) r += (k/u) * calc(unopen-1, known-1);
	r += ((u-k)/u) * (1.0/(u-1)) * calc(unopen-2, known);
	if (known > 0) r += ((u-k)/u)*(k/(u-1)) * (calc(unopen-2, known)+1);
	if (unopen - known > 2) r += ((u-k)/u)*((u-k-2)/(u-1)) * (calc(unopen-2, known+2)+1);

	f[unopen][known] = 1;
	m[unopen][known] = r;
	return r;
}

int main()
{
	int n;
	
	memset(f[0], 1, sizeof(f[0]));
	while (scanf("%d", &n) && n > 0) printf("%.10lf\n", calc(n, 0));
	return 0;
}