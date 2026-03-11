// AOJ 3017: Piano
// 2017.12.27 bal4u@uu

#include <stdio.h>

char buf[12], *p;

int in()
{
	int n = 0;
	while (*p >= '0') n = 10*n + (*p++ & 0xf);
	return n;
}

int main()
{
	int n, a, a0, d, w, ans;

	fgets(p=buf, 12, stdin), n = in();
	ans = d = 0;
	fgets(p=buf, 12, stdin), a0 = in(), w = 1;
	while (--n) {
		fgets(p=buf, 12, stdin), a = in();
		if (a > a0) {
			if (d == 1) w++;
			else { if (w > ans) ans = w; w = 2, d = 1; }
		} else if (a < a0) {
			if (d == -1) w++;
			else { if (w > ans) ans = w; w = 2, d = -1;	}
		}
		a0 = a;
	}
	if (w > ans) ans = w;
	printf("%d\n", ans);
	return 0;
}