// Aizu 1107: Spiral Footrace
// 2017.9.16 bal4u@uu

#include <stdio.h>
#include <string.h>
#include <math.h>

#define EPS 1e-6

int x[405], y[405];
char f[405];
double ans;

double theta(int x1, int y1, int x2, int y2)
{
    int dx, dy, ax, ay;
    double t;

    dx = x2 - x1;
    ax = dx < 0 ? -dx : dx;
    dy = y2 - y1;
    ay = dy < 0 ? -dy : dy;
    t = (ax + ay == 0) ? 0 : (double)dy/(double)(ax+ay);
    if (dx < 0) t = 2 - t;
    else if (dy < 0) t = 4 + t;
    return t * 90.0;
}

void convexHull(int pn, int *x, int *y)
{
    int i, j, px, py;
    int min;
    double minang, nowang;
    double dist, d, rad;

    min = 0, minang = 90, px = py = 0;
    for (j = 0; j < pn; j++) {
        dist = 0x7ffffff, nowang = minang; minang = -360;
		if (nowang < 0) nowang += 360;
        for (i = 0; i < pn; i++) {
			if (f[i]) continue;
            d = sqrt((x[i]-px)*(x[i]-px)+(y[i]-py)*(y[i]-py));
            rad = theta(px, py, x[i], y[i]);
	        if (d == 0) continue;
			if (rad > nowang) rad -= 360;
            if (rad < nowang+EPS) {
                if (rad - EPS > minang) {
                    dist = d;
                    minang = rad;
                    min = i;
                } else if (fabs(rad-minang) <= EPS && d < dist) {
                    dist = d;
                    minang = rad;
                    min = i;
                }
            }
#if 0
			else if (fabs(rad-nowang) < EPS && d > EPS &&
                       fabs(rad-minang) > EPS || d < dist) {
                dist = d;
                minang = rad;
                min = i;
            }
#endif
        }
		f[min] = 1;
		ans += sqrt((px-x[min])*(px-x[min])+(py-y[min])*(py-y[min]));
		px = x[min], py = y[min];
    }
}

int main()
{
	int n, i;

	while (scanf("%d", &n) && n) {
		memset(f, 0, sizeof(f));
		for (i = 0; i < n; i++) scanf("%d%d", x+i, y+i);
		ans = 0; convexHull(n, x, y);
		printf("%.1lf\n", ans);
	}
	return 0;
}