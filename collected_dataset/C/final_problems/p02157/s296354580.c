// AOJ 3059 Shuffle 2
// 2019.4.2 bal4u

#include <stdio.h>
#include <stdlib.h>

// 高速文字表示
#if 1
#define pc(c) putchar_unlocked(c)
#else
#define pc(c) putchar(c)
#endif
void outs(char *s) { while (*s) pc(*s++); }

char ans[2000010], b[2000010];

int main()
{
	int q, i;
	long long n, k, d;
	long long d2, diff;

	scanf("%lld%d%lld%lld", &n, &q, &k, &d);
	k--, d--;

	d2 = d;  for (i = 0; i < q; i++) d2 = (d2 << 1) % n;
	diff = (k - d2) % n;
	if (diff < 0) diff += n;
	for (i = q-1; i >= 0 && diff; i--) b[i] = diff & 1, diff >>= 1;
	if (diff) { outs("-1\n"); return 0; }

	for (i = 0; i < q; i++) {
		d2 = d << 1;
		if (b[i]) d2++;
		d2 %= n;
		ans[i] = (d2 & 1) ^ (d != (d2 >> 1));
		d = d2;
	}
	for (i = q - 1; i >= 0; i--) pc('0' + ans[i]), pc('\n');
	return 0;
}

