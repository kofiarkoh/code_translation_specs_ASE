// AOJ 1084: K Cards
// 2018.2.3 bal4u

#include <stdio.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int n, k, m;
int c[101];

int check()
{
	int i, j, t, max = 0;

	for (i = 0; i <= m; i++) {
		t = c[i];
		for (j = 1; j < k; j++)	t *= c[i+j];
		if (t > max) max = t;
	}
	return max;
}

int main()
{
	int i, j, x, t, max0, max;

	while (n = in()) {
		k = in(), m = n-k;
		for (i = 0; i < n; i++) c[i] = in();

		max0 = check();
		max = 0;
		for (i = 0; i < n; i++) for (j = i+1; j < n; j++) {
			x = c[i], c[i] = c[j], c[j] = x;
			t = check();
			if (t > max) max = t;
			x = c[i], c[i] = c[j], c[j] = x;
		}
		max -= max0;
		if (max >= 0) printf("%d\n", max);
		else puts("NO GAME");
	}
	return 0;
}
