#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#define Rep(i,a,n)for(int i=(int)(a);i<(int)(n);i++)
#define rep(i,n)Rep(i,0,n)
typedef long long lld;
typedef long double Lf;
#define Ma(x,y)((x)>(y)?(x):(y))
#define mi(x,y)((x)<(y)?(x):(y))

int main() {
	int n, k, t, all[6] = { 0 }, d[10] = { 0 };
	scanf("%d%d", &n, &k);
	rep(i, k) { scanf("%d", &t); d[t]++; }
	if (n == 1 && d[1] != 1) { printf("1"); return 0; }
	t++;
	Rep(i, 2 < n ? n : 2, 1e5) {
		rep(j, log10(i) + 1) {
			all[j] = (i % (int)pow(10, j + 1)) / (int)pow(10, j);
			rep(x, 10)if (d[all[j]])t = 0;
		}
		if (t) { printf("%d", i); return 0; }
		t++;
	}
}