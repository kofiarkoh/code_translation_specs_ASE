// AOJ 1067 Cutting a Chocolate
// 2018.3.17 bal4u

#include <stdio.h>
#include <stdlib.h>

//#define getchar_unlocked()  getchar()
int in()        // get a non-negative integer from stdin
{
	int n = 0, c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

double dbl()    // get a non-negative real number from stdin  
{
	double x, y;
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	x = n;
	if (c == '.') {
		y = 1, c = getchar_unlocked();
		do y *= 0.1, x += y * (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	}
	return x;
}

int hl[30002], hr[30002], s[30002], a[30002];

int binary_search(int *tbl, int hi, int k, int key)
{
	int lo = 0, mi, r;
	
	while (lo+1 < hi) {
		mi = (lo + hi) >> 1;
		r = tbl[mi] * k - key;
		if (!r) return mi;
		if (r < 0) lo = mi; else hi = mi;
	}
	return lo;
}

int main()
{
	int n, m, w, h, S, ans;
	int i, j;
	double x, y, y1, y2;

	while (n = in()) {
		m = in(), w = in(), h = in(), S = (w*h-in()) << 1;
		for (i = 0; i < m; i++) {
			hl[i] = in(), hr[i] = in(), a[i] = 0;
			s[i] = hl[i] + hr[i];
		}
		for (i = 0; i < n; i++) {
			x = dbl(), y = dbl();
			j = binary_search(hl, m, 1, (int)y);
			y1 = (hr[j] - hl[j]) * x / w + hl[j], j++;
			while (1) {
				y2 = (hr[j] - hl[j]) * x / w + hl[j];
				if (y1 < y && y < y2) break;
				y1 = y2, j++;
			}
			a[j]++;
		}
		for (i = 1; i < m; i++) a[i] += a[i-1];

		if (S == 0) { printf("%d\n", n); continue; }
		else if (S == 2*w*h) { puts("0"); continue; }

		j = binary_search(s, m, w, S); ans = a[j++];
		i = 0; while (j < m) {
			while ((s[j] - s[i])*w > S) i++;
			if (a[j] - a[i] > ans) ans = a[j] - a[i];
			j++;
		}
		printf("%d\n", n - ans);
	}
	return 0;
}

