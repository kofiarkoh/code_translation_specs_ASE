// AOJ 1336: The Last Ant
// 2017.12.16 bal4u@uu

#include <stdio.h>
#include <string.h>

int l[102], r[102];

int main()
{
	int N, L, i, p, t, last;
	char buf[5];

	while (scanf("%d%d", &N, &L) && N > 0) {
		memset(l, 0, sizeof(l)), memset(r, 0, sizeof(r));
		for (i = 1; i <= N; i++) {
			scanf("%s%d", buf, &p);
			if (*buf == 'L') l[p] = i;
			else             r[p] = i;
		}

		for (last = 0, t = -1; N > 0; t++) {
			if (r[L]) last = r[L], r[L] = 0, N--;
			for (i = L-1; i >= 0; i--) r[i+1] = r[i];

			if (l[0]) last = l[0], l[0] = 0, N--;
			for (i = 1; i <= L; i++) {
				if (!l[i]) continue;
				if (r[i-1]) {
					l[i-1] = r[i-1], r[i-1] = l[i];
				} else l[i-1] = l[i];
				l[i] = 0;
			}
		}
		printf("%d %d\n", t, last);
	}
	return 0;
}