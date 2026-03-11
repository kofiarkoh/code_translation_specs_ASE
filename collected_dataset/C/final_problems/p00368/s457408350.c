// AOJ 0373: Checkered Pattern
// 2017.11.14 bal4u@uu

#include <stdio.h>

int s[1001];
char buf[2100], *p;

int check(int sum, int one, int zero)
{
	if (sum & 1) return one == zero+1 || one == zero-1;
	else         return one == zero;
}

int main()
{
	int w, h, r, c, one, ans;

	fgets(buf, 20, stdin);
	sscanf(buf, "%d%d", &w, &h);
	ans = 0;
//	memset(s, 0, sizeof(s));
	for (one = 0, r = 0; r < h; r++) {
		int s1 = 0;
		fgets(p=buf, 2100, stdin);
		for (c = 0; c < w; c++, p+=2) {
			if (*p & 1) one++, s[c]++, s1++;
		}
		if (!check(w, s1, w-s1)) goto done;
	}
	for (c = 0; c < w; c++) if (!check(h, s[c], h-s[c])) goto done;
	ans = check(w*h, one, w*h - one);
done:
	puts(ans ? "yes" : "no");
	return 0;
}