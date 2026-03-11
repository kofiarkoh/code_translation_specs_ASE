// AOJ 1552: Mountain Climbing
// 2019.2.21 bal4u

#include <stdio.h>
#include <stdlib.h>

#define gc() getchar()
int in()
{
	int n = 0, c = gc();
	if (c == '-') {
		c = gc();
		do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
		return -n;
	}
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int cnt[5];

int main()
{
	int N, aa, a, f, zero;

	N = in();
	f = zero = 0;
	aa = in();
	while (--N) {
		a = in();
		if (a < aa) {
			if (f == 1) {
				if (zero) cnt[0]++;
				else cnt[3]++;
			}
			else if (f == -1) {
				if (zero) cnt[2]++;
			}
			f = -1, zero = 0;
		}
		else if (a > aa) {
			if (f == 1) {
				if (zero) cnt[2]++;
			}
			else if (f == -1) {
				if (zero) cnt[1]++;
				else cnt[4]++;
			}
			f = 1, zero = 0;
		}
		else zero = 1;
		aa  = a;
	}
	printf("%d %d %d %d %d\n", cnt[0], cnt[1], cnt[2], cnt[3], cnt[4]);
	return 0;
}
