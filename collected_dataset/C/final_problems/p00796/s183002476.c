// AOJ 1216: Lost in Space
// 2017.10.15

#include <stdio.h>
#include <math.h>

typedef struct { double x, y, z; } PP;
PP pp[33];

double qr0, rp0, pq0;

double dist(PP a, PP b)
{
	double dx, dy, dz;
	dx = a.x - b.x, dy = a.y - b.y, dz = a.z - b.z;
	return sqrt(dx*dx + dy*dy + dz*dz);
}

int check(double a, double b) { return fabs(a-b) < 0.1; }

int main()
{
	int t, n, i, p, q, r;
	double pq, qr, rp;

	scanf("%d", &t);
	while (t-- > 0) {
		scanf("%lf%lf%lf", &qr0, &rp0, &pq0);
		scanf("%d", &n);
		for (i = 0; i < n; i++) scanf("%lf%lf%lf", &pp[i].x, &pp[i].y, &pp[i].z);

		for (p = 0; p < n; p++) for (q = 0; q < n; q++) if (p != q) for (r = 0; r < n; r++) {
			if (r == p || r == q) continue;
			pq = dist(pp[p], pp[q]);
			qr = dist(pp[q], pp[r]);
			rp = dist(pp[r], pp[p]);
			if (check(qr/qr0, rp/rp0) && check(rp/rp0, pq/pq0) && check(pq/pq0, qr/qr0)) {
				printf("%d %d %d\n", p+1, q+1, r+1);
                goto Done;
			}
		}
		Done:;
	}
	return 0;
}