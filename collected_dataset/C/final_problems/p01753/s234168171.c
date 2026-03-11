#include <stdio.h>

#define sq(x) ((x) * (x))

#define EPS 1e-9

int main(void) {
	int i;
	int n, q, x[50], y[50], z[50], r[50];
	long long l[50];

	scanf("%d %d", &n, &q);
	for (i = 0; i < n; i++)
		scanf("%d %d %d %d %lld", x + i, y + i, z + i, r + i, l + i);

	while (q--) {
		int xa, ya, za, xb, yb, zb;
		long long ans;

		scanf("%d %d %d %d %d %d", &xa, &ya, &za, &xb, &yb, &zb);

		ans = 0;
		for (i = 0; i < n; i++) {
			const double k = (double)((x[i] - xa) * (xb - xa) + (y[i] - ya) * (yb - ya) + (z[i] - za) * (zb - za)) / (sq(xb - xa)  + sq(yb - ya) + sq(zb - za));
			const double len_sq = sq(xa + k * (xb - xa) - x[i]) + sq(ya + k * (yb - ya) - y[i]) + sq(za + k * (zb - za) - z[i]);
			if (len_sq - sq(r[i]) < EPS)
				ans += l[i];
		}

		printf("%lld\n", ans);
	}

	return 0;
}