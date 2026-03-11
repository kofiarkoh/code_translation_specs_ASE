// AOJ 1032 Course Planning for Lazy Students
// 2018.2.28 bal4u

#include <stdio.h>
#include <string.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

typedef struct { int s, c; } Q;
Q q[10000]; int top, tail;
int c[22], r[22];
char mk[1<<20];

int bitcount(int bits)
{
    bits = (bits & 0x55555555) + (bits >> 1 & 0x55555555);
    bits = (bits & 0x33333333) + (bits >> 2 & 0x33333333);
    bits = (bits & 0x0f0f0f0f) + (bits >> 4 & 0x0f0f0f0f);
    bits = (bits & 0x00ff00ff) + (bits >> 8 & 0x00ff00ff);
    return (bits & 0x0000ffff) + (bits >>16 & 0x0000ffff);
}

int main()
{
	int n, U, i, k, s;
	
	while (n = in()) {
		U = in();
		for (i = 0; i < n; i++) {
			c[i] = in(), r[i] = 0;
			k = in(); while (k--) r[i] |= 1 << in();
		}

		memset(mk, 0, sizeof(mk));
		q[0].s = q[0].c = 0, top = 0, tail = 1;
		while (top != tail) {
			s = q[top].s, k = q[top++].c;
			if (mk[s]) continue;
			mk[s] = 1;
			if (k >= U) break;
			for (i = 0; i < n; i++) if (!(s & (1<<i)) && (r[i] & s) == r[i]) {
				q[tail].s = s | (1<<i), q[tail++].c = k + c[i];
			}
		}
		printf("%d\n", bitcount(s));
	}
	return 0;
}
