// AOJ 1366 Min-Max Distance Game
// 2018.2.8 bal4u

#include <stdio.h>

#define INF 0x30303030

int x[100002], n;

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
//	while (c <= ' ') c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int check(int t)
{
	int i, j, f = 1;
	for (j = 0, i = 0; i < n; i++) if (x[i]-x[j] > t) f++, j = i;
	return f;
}

int main()
{
	int m, alice, i, j, ans;
	int lo, hi, mi;

	n = in(), m = (n+1) >> 1;
	alice = (getchar_unlocked() == 'A');
	i = 3; if (alice) i += 2;
	while (i--) getchar_unlocked();
	for (i = 0; i < n; i++) x[i] = in();

	ans = INF;
	if ((n & 1) == alice) {
		for (i = 0, j = m; j < n; i++, j++) if (x[j]-x[i] < ans) ans = x[j]-x[i];
	} else {
		lo = 0, hi = x[n-1];
		while (hi-lo > 1) {
			mi = (lo+hi) >> 1;
			if (check(mi) > m) lo = mi; else hi = mi;
		}
		ans = hi;
	}
	printf("%d\n", ans);
	return 0;
}
