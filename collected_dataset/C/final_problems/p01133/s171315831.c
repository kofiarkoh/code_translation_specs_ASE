// AOJ 2008: Dragon Fantasy
// 2017.11.22 bal4u@uu

#include <stdio.h>
#include <string.h>
#include <math.h>

#define dist(x, y)	(hypot(x, y))

int cx[22], cy[22], n, lim;
double dd[22];
double dc[22][22];

int rec(int i, int s, double d)
{
	int j, jj;

	if (s == lim) return 1;
	for (jj = 1, j = 0; j < n; j++, jj <<= 1) {
		if (s & jj) continue;
		if (dd[j] <= d + dc[i][j]) return 0;
	}
	for (jj = 1, j = 0; j < n; j++, jj <<= 1) {
		if (s & jj) continue;
		if (rec(j, s | jj, d + dc[i][j])) return 1;
	}
	return 0;
}

int main()
{
	int i, j, hx, hy, dx, dy;

	while (scanf("%d%d%d%d%d", &n, &hx, &hy, &dx, &dy) && n > 0) {
		n++; lim = (1<<n)-1;
		cx[0] = hx, cy[0] = hy;
		for (i = 1; i < n; i++) scanf("%d%d", cx+i, cy+i);
		for (i = 0; i < n; i++) {
			dd[i] = dist(cx[i]-dx, cy[i]-dy);
			for (j = i+1; j < n; j++) {
				dc[i][j] = dc[j][i] = dist(cx[i]-cx[j], cy[i]-cy[j]);
			}
		}
		puts(rec(0, 0, 0) ? "YES" : "NO");
	}
	return 0;
}