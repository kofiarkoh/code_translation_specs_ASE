// AOJ 2165: Strange String Manipulation
// 2017.10.6 bal4u@uu

#include <stdio.h>
#include <string.h>
#include <math.h>

#define EPS 1.0e-8
#define M 255

int t[257];
int f[256];

int main()
{
	int n, s, a, c, i, r, ss, aa, cc;
	double h, hh;

	while (scanf("%d", &n) && n > 0) {
		for (i = 0; i < n; i++) scanf("%d", t+i); 

        hh = 1.0e8;
		for (s = 0; s <= 15; s++) for (a = 0; a <= 15; a++) for (c = 0; c <= 15; c++) {
			memset(f, 0, sizeof(f));
			r = s;
            for (i = 0; i < n; i++) r = (a * r + c) & M, f[(t[i] + r) & M]++;
            h = 0;
			for (i = 0; i <= M; i++) if (f[i]) {
				double x = (double)f[i] / n;
				h -= x * log(x);
			}
	        if (h + EPS < hh) hh = h, ss = s, aa = a, cc = c;
        }
		printf("%d %d %d\n", ss, aa, cc);
    }
	return 0;
}