// AOJ ITP2_10_B Bit Operation II
// 2019.3.8 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif

unsigned in()   // 非負整数の入力
{
	unsigned n = 0;
	int c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void outb(unsigned n)   // 非負整数の2進数表示、最後に改行あり
{
	int i;
	char ob[40];

	i = 0; while (i < 32) ob[i++] = (n & 1) + '0', n >>= 1;
	while (i--) pc(ob[i]);
	pc('\n');
}

int main()
{
	unsigned a, b;

	a = in(), b = in();
	outb(a & b);
	outb(a | b);
	outb(a ^ b);
	return 0;
}
