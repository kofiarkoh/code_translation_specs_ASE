// AOJ 2642: Dinner
// 2017.11.9 bal4u@uu
// 2018.1.5

#include <stdio.h>
#include <stdlib.h>

long long a[500003];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	if (c == '-') {
		c = getchar_unlocked();
		do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
		while (c >= '0');
		return -n;
	}
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

#define THRESHOLD 10
#define STACKSIZE 64		// 2^432 for long long int

void quicksort(long long *a, int n)
{
	int i, j, l, r, p;
	int lsp[STACKSIZE], rsp[STACKSIZE];
	long long m, t;
	
	l = 0, r = n-1, p = 0;
	while (1) {
		if (r-l <= THRESHOLD) {
			if (!p--) break;
			l = lsp[p], r = rsp[p];
		}
		m = a[(l+r)>>1];
		i = l, j = r;
		while (1) {
			while (a[i] > m) i++;
			while (m > a[j]) j--;
			if (i >= j) break;
			t = a[i], a[i] = a[j], a[j] = t;
			i++, j--;
		}
		if (i-l > r-j) {
			if (i-l > THRESHOLD) {
				lsp[p] = l;
				rsp[p] = i-1;
				p++;
			}
			l = j+1;
		} else {
			if (r-j > THRESHOLD) {
				lsp[p] = j+1;
				rsp[p] = r;
				p++;
			}
			r = i-1;
		}
	}
	
	// insert sorting
	for (i = 1; i < n; i++) {
		m = a[i];
		for (j = i-1; j >= 0 && a[j] < m; j--) a[j+1] = a[j];
		a[j+1] = m;
	}
}


int main()
{
	int n, q, c, i;
	long long p, s, t, ans;

	n = in(), p = in(), q = in();
    for (s = 0, t = p*q, i = 0; i < n; i++, t -= p) {
		c = in(), a[i] = t - c, s += c;
    }

	quicksort(a, n);
    for (ans = s, t = 0, p <<= 1, i = 0; i < n; i++, t += p) {
		s += a[i] + t;
		if (s > ans) ans = s;
	}
    printf("%lld\n", ans);
	return 0;
}
