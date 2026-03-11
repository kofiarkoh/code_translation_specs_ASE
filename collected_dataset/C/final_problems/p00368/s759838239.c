// AOJ 0373: Checkered Pattern
// 2017.11.14 bal4u@uu

#include <stdio.h>

int s[1001];
char buf[2500], *p;

int check(int one, int zero)
{
	return zero == one || one == zero+1 || zero == one+1;
}

int main()
{
	int w, h, r, c, one, ans;

	fgets(buf, 20, stdin);
	sscanf(buf, "%d%d", &w, &h);
	one = 0;
	ans = 0;
//	memset(s, 0, sizeof(s));
	for (r = 0; r < h; r++) {
		int s1 = 0;
		fgets(buf, 2500, stdin);
		for (p=buf, c = 0; c < w; c++, p+=2) {
			if (*p & 1) one++, s[c]++, s1++;
		}
		if (!check(s1, w-s1)) goto done;
	}
	for (c = 0; c < w; c++) if (!check(s[c], h-s[c])) goto done;
	ans = check(one, w*h - one);
done:
	puts(ans ? "yes" : "no");
	return 0;
}