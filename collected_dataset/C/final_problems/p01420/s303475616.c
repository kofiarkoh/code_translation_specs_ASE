// AOJ 2303 Marathon Match
// 2018.4.11 bal4u

#include <stdio.h>
#include <math.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

#define EPS 1e-8
int N, M;
int p[102], t[102], v[102];
double comb[52][52];
double ps[102][52], ss[102][52], ts[102][52];

int binarySearch(double *a, double x)
{
	int m, l = 0, r = M+1;

    while (l < r) {
        m = (l+r) >> 1;
        if (a[m] < x+EPS) l = m + 1; else r = m;
    }
	return l;
}

int main()
{
	int i, j, k, f;
	double L, pp, x, ans;

	for (i = 0; i <= 50; i++) comb[i][0] = 1;
	for (i = 1; i <= 50; i++) for (j = 1; j <= i; j++)
		comb[i][j] = comb[i-1][j-1] + comb[i-1][j];

	N = in(), M = in(), L = in();
	f = 0; 
	for (i = 0; i < N; i++) {
		p[i] = in(), t[i] = in(), v[i] = in();
		if (v[i]) f = 1;
	}
	if (!f) {
		for (i = 0; i < N; i++) puts("0.00000000");
		return 0;
	}


	for (i = 0; i < N; i++) for (j = 0; j <= M; j++) {
		ps[i][j] = pow(p[i]/100.0, j) * pow((100-p[i])/100.0, M-j) * comb[M][j];
		if (!j) ss[i][j] = ps[i][j];
		else    ss[i][j] = ss[i][j-1] + ps[i][j];
		ts[i][j] = L/v[i] + t[i]*j;
	}

	for (i = 0; i < N; i++) {
		ans = 0;
		for (j = 0; j <= M; j++) {
			pp = 1, x = ts[i][j];
			for (k = 0; k < N; k++) if (k != i) {
				if (f = binarySearch(ts[k], x)) pp *= 1 - ss[k][f-1];
			}
			ans += ps[i][j] * pp;
		}
		printf("%.8f\n", ans);
	}
	return 0;
}

