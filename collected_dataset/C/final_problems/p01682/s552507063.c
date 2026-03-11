// AOJ 2570: Shipura
// 2017.12.1 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

#define M  1000000007

char buf[2000002], *p;

unsigned parse();

unsigned factor()
{
	unsigned n;
	unsigned long long x;

	while (*p == ' ') p++;
	if (*p == 'S') {
		while (*++p == ' ');
		x = parse(++p);
		x = x*x % M;
		while (*p == ' ') p++;
		p++;
		return (unsigned)x;
	} else {
		n = 0; while (isdigit(*p)) n = (n<<3)+(n<<1) + (*p++ & 0xf);
		return n;
	}
}

unsigned parse()
{
	unsigned x, y;

	x = factor();
	while (*p) {
		while (*p == ' ') p++;
		if (*p != '!') break;
		p++, y = factor();
		if (y > 31) x = 0; else x = x >> y;
	}
	return x;
}

int main()
{
	while (fgets(buf+1, sizeof(buf), stdin) && *(buf+1) != '#') {
		p = buf, *p++ = 0; while (*p >= ' ') *p++; *p-- = 0;
		while (*p) {
			if (!isdigit(*p)) { p--; continue; }
			while (isdigit(*p)) p--;
			while (*p == ' ') p--;
			if (*p == '>' && *(p-1) == '>') *p = '!', *(p-1) = ' ', p-=2;
		}
		p++, printf("%u\n", parse());
	}
	return 0;
}