// Aizu Vol-1 0179: Mysterious Worm
// 2017.8.22 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX 0xaaaaa
char chk[MAX+10];
int Q[MAX+10], *top, *end, *qmax = Q + MAX;
int N, K, mask;

int nxt[16] = { 0, 10, 5, 0, 10, 5, 0, 0, 5, 0, 10 };

int main()
{
	int s, r, g, b, step;
	int i, k;
	char buf[15], *bp;

	while (1) {
		scanf("%s", buf); if (buf[0] == '0') break;
		memset(chk, 0, sizeof(chk));
		for (N = s = 0, bp = buf; *bp; bp++, N += 2) {
			if      (*bp == 'r') k = 0;
			else if (*bp == 'g') k = 1;
			else                 k = 2;
			s = (s << 2) | k;
		}
		mask = (1 << N) - 1, r = 0, g = 0x55555 & mask, b = 0xaaaaa & mask;
		top = end = Q;
		*end++ = s, *end++ = -1, chk[s] = 1, step = 0, K = N-4;
		while (top < end) {
			s = *top; if (++top >= qmax) top = Q;
			if (s < 0) { step++, *end = -1; if (++end >= qmax) end = Q; continue; }
			if (s == r || s == g || s == b) { printf("%d\n", step); break; }
			for (i = 0; i <= K; i += 2) {
				int s2, k2;
				k = (s >> i) & 0xf;
				k2 = nxt[k];
				if (k2 == k) continue;
				s2 = (s & ~(0xf << i)) | (k2 << i);
				if (!chk[s2]) { chk[s2] = 1, *end = s2; if (++end >= qmax) end = Q; }
			}
		}
		if (top >= end) puts("NA");
	}
	return 0;
}