// AOJ 2300: Calender Colors
// 2017.10.7 bal4u@uu

#include <stdio.h>
//#include <string.h>

double l[21], a[21], b[21]; int n, m;
int one, lim;
 
double calc(unsigned x)
{
	int i, j, s, t;
	double sm = 0;

	if (one) {
		for (s = 0, i = 1; i < lim; i <<= 1, s++) {
			if (x & i) for (t = s+1, j = i << 1; j < lim; j <<= 1, t++) {
				if (x & j) {
					sm += (l[s]-l[t])*(l[s]-l[t]) + (a[s]-a[t])*(a[s]-a[t])
						+ (b[s]-b[t])*(b[s]-b[t]);
				}
			}
		}
	} else {
		for (s = 0, i = 1; i < lim; i <<= 1, s++) {
			if (!(x & i)) for (t = s+1, j = i << 1; j < lim; j <<= 1, t++) {
				if (!(x & j)) {
					sm += (l[s]-l[t])*(l[s]-l[t]) + (a[s]-a[t])*(a[s]-a[t])
						+ (b[s]-b[t])*(b[s]-b[t]);
				}
			}
		}
	}
	return sm;
}

unsigned next(unsigned x)
{
	unsigned min, rip, new, one;

	min = x & -x;
	rip = x + min;
	new = rip & -rip;
	one = ((new / min) >> 1) - 1;
	return rip | one;
}

int main()  
{  
	int i;
	unsigned x, x0;
	double s, ans;

	scanf("%d%d", &n, &m);
	if (m == 0) { puts("0.00000000000000000000"); return 0; }
	for (i = 0; i < n; i++) scanf("%lf%lf%lf", l+i, a+i, b+i);
	lim = 1 << n, one = 1;
	if (m < n && m > n/2) m = n - m, one = 0;

	ans = 0;
	x = (1U << m) - 1U;
	x0 = ~((1U << n) - 1U);
	while (!(x & x0)) {
		if ((s = calc(x)) > ans) ans = s;
		x = next(x);
	}
	printf("%.20lf\n", ans);
	return 0;
}