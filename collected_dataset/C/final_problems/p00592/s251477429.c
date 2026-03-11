// Aizu Vol-10 1006:   Boring Commercials
// 2017.8.19 bal4u@uu

#include <stdio.h>
#include <string.h>

int tbl[1500];

int d2t(int d) { return (d / 100) * 60 + (d % 100); }

int main()
{
	int n, p, q, k, t1, t2;
	int i, j, t, max;

	while (scanf("%d%d%d", &n, &p, &q) && n) {
		memset(tbl, 0, sizeof(tbl));
		p = d2t(p), q = d2t(q);
		for (i = 0; i < n; i++) {
			scanf("%d", &k);
			for (j = 0; j < k; j++) {
				scanf("%d%d", &t1, &t2);
				t1 = d2t(t1), t2 = d2t(t2);
				for (t = t1; t < t2; t++) tbl[t]++;
			}
		}
		for (max = 0, k = 0, t = p; t < q; t++) {
			if (tbl[t] < n) k++;
			else if (k > 0) { if (k > max) max = k; k = 0; }
		}
		if (k > max) max = k;
		printf("%d\n", max);
	}
	return 0;
}