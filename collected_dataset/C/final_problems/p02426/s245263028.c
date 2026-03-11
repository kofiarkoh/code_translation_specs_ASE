// AOJ ITP2_10_D Bit Mask
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

int in()   // 非負整数の入力
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(unsigned long long n)   // 非負整数の表示、最後に改行あり
{
	int i;
	char ob[20];

	if (!n) pc('0');
	else {
		i = 0; while (n) ob[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(ob[i]);
	}
	pc('\n');
}

int bitcount64(unsigned long long x)
{
	x = ((x & 0xAAAAAAAAAAAAAAAAULL) >> 1) + (x & 0x5555555555555555ULL);
	x = ((x & 0xCCCCCCCCCCCCCCCCULL) >> 2) + (x & 0x3333333333333333ULL);
	x = ((x & 0xF0F0F0F0F0F0F0F0ULL) >> 4) + (x & 0x0F0F0F0F0F0F0F0FULL);
	x = ((x & 0xFF00FF00FF00FF00ULL) >> 8) + (x & 0x00FF00FF00FF00FFULL);
	x = ((x & 0xFFFF0000FFFF0000ULL) >> 16) + (x & 0x0000FFFF0000FFFFULL);
	x = ((x & 0xFFFFFFFF00000000ULL) >> 32) + (x & 0x00000000FFFFFFFFULL);
	return (int)x;
}

int N;
unsigned long long mask[12];

int main()
{
	int i, q, c;
	unsigned long long n, m;

	N = in();
	for (i = 0; i < N; i++) {
		c = in();
		while (c--) mask[i] |= 1LL << in();
	}
	q = in(), n = 0;
	while (q--) {
		c = gc() & 0xf, gc(), i = in(), m = mask[i];
		switch (c) {
		case 0: pc((n & (1LL << i)) ? '1' : '0'), pc('\n'); break;
		case 1: n |= m; break;
		case 2: n &= ~m; break;
		case 3: n ^= m; break;
		case 4: pc((n & m) == m ? '1' : '0'), pc('\n'); break;
		case 5: pc((n & m)? '1' : '0'), pc('\n'); break;
		case 6: pc(!(n & m) ? '1' : '0'), pc('\n'); break;
		case 7: out(bitcount64(n & m)); break;
		case 8: out(n & m); break;
		}
	}
	return 0;
}
