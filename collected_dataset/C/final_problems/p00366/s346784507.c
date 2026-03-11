// AOJ 0371: Electronic Metronome
// 2017.11.14 bal4u@uu

#include <stdio.h>

int t[100002];

char buf[10], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

int main()
{
	int n, i, x, y, max, ans;

	fgets(p=buf, 10, stdin);
	n = getint();
	max = 0;
	for (i = 0; i < n; i++) {
		fgets(p=buf, 10, stdin);
		t[i] = getint();
		if (t[i] > max) max = t[i];
	}
	ans = 0;
	if (max & 1) y = max/3;
	else         y = max/2;
	for (i = 0; i < n; i++) {
		for (x = t[i]; ; x++) {
			if (x > y)     { ans += max - t[i]; break; }
			if (max % x == 0) { ans += x - t[i]; break; }
		}
	}
	printf("%d\n", ans);
	return 0;
}