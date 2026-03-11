// AOJ 0328: Metal Recycling
// 2017.10.14

#include <stdio.h>

char buf[30], *p;
int f[200020];

int getInt()
{
	int n = 0;
	if (*p == ' ') p++;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

int main()
{
	int n, i, max;

	fgets(p=buf, 10, stdin); n = getInt();
	max = 0; while (n-- > 0) {
		fgets(p=buf, 30, stdin);
		i = getInt() + getInt();
		if (i > max) max = i;
		f[i]++;
	}
	for (i = 0; i <= max; i++) {
		if (f[i] >= 1) {
			f[i+1] += f[i] >> 1, f[i] &= 1;
			if (i == max) max++;
		}
	}
	for (i = 0; i <= max; i++) if (f[i]) printf("%d 0\n", i);
	return 0;
}