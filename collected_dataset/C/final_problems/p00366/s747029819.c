// AOJ 0371: Electronic Metronome
// 2017.11.14 bal4u@uu

#include <stdio.h>

int t[100002];
int a[100000], sz;

char buf[10], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

int bsch(int x)
{
	int m, l = 0, r = sz;

    while (l < r) {
        m = (l + r) >> 1;
        if (a[m] < x) l = m + 1; else r = m;
    }
	return a[l]-x;
}

int main()
{
	int n, i, max, ans;

	fgets(p=buf, 10, stdin), n = getint();
	for (max = 0, i = 0; i < n; i++) {
		fgets(p=buf, 10, stdin);
		t[i] = getint();
		if (t[i] > max) max = t[i];
	}

	for (sz = 0, i = 1; i <= max; i++) if (max % i == 0) a[sz++] = i;

	for (ans = 0, i = 0; i < n; i++) ans += bsch(t[i]);

	printf("%d\n", ans);
	return 0;
}