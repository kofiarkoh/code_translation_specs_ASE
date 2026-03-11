// AOJ 0328: Metal Recycling
// 2017.10.14

#include <stdio.h>

char buf[32], *p;
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
	int *q;

	fgets(p=buf, 16, stdin); n = getInt();
	max = 0; while (n--) {
		fgets(p=buf, 32, stdin);
		i = getInt() + getInt();
		if (i > max) max = i;
		f[i]++;
	}
	for (q = f, i = 0; i <= max; i++, q++) {
		if (*q > 0) {
			*(q+1) += *q >> 1;
			if (*q & 1) printf("%d 0\n", i);
			if (i == max) max++;
		}
	}
	return 0;
}