// AOJ 2334 Roads on Towns
// 2018.2.24 bal4u

#include <stdio.h>
#include <math.h>

#define EPS			1e-10
#define INF			1e20
typedef struct { double x, y; } PP;

PP p[2][1002];

PP vectorSub(PP *p1, PP *p2) { PP r; r.x = p1->x - p2->x, r.y = p1->y - p2->y; return r; }
double cross(PP *a, PP *b) { return a->x * b->y - a->y * b->x; }
double dot(PP *a, PP *b) { return a->x * b->x + a->y * b->y; }
double norm(PP *a) { return a->x * a->x + a->y * a->y; }

int ccw(PP *p0, PP *p1, PP *p2)
{
	PP a = vectorSub(p1, p0), b = vectorSub(p2, p0);
	double t = cross(&a, &b);
	if (t >  EPS) return 1;
	if (t < -EPS) return -1;
	if (dot(&a, &b) < -EPS) return 2;
	if (norm(&a) < norm(&b)) return -2;
	return 0;
}

int intersect(PP *p1, PP *p2, PP *p3, PP *p4)
{
	return ccw(p1, p2, p3) * ccw(p1, p2, p4) <= 0 &&
		   ccw(p3, p4, p1) * ccw(p3, p4, p2) <= 0;
}

double dist(PP *p1, PP *p2) { return hypot(p1->x - p2->x, p1->y - p2->y); }

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	if (c == '-') {
		c = getchar_unlocked();
		do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
		while (c >= '0');
		return -n;
	}
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

double calc(PP *s, PP *g, PP *a, int n, PP *es, PP *eg)
{
	int i, j;
	double ans, x, y, z;

	if (!intersect(s, g, es, eg)) return dist(s, g);
	if (n == 0) return INF+100;

	ans = INF+100;
	for (i = 0; i < n; i++) {
		if (intersect(s, a+i, es, eg)) continue;
		x = dist(s, a+i);
		if (x >= ans) continue;
		if (!intersect(g, a+i, es, eg)) {
			y = dist(g, a+i);
			if (x + y < ans) ans = x + y;
			continue;
		}
		for (j = 0; j < n; j++) if (j != i) {
			if (intersect(a+i, a+j, es, eg)) continue;
			y = dist(a+i, a+j);
			if (x + y  >= ans) continue;
			if (!intersect(g, a+j, es, eg)) {
				z = dist(g, a+j);
				if (x + y + z < ans) ans = x + y + z;
			}
		}
	}
	return ans;
}

int main()
{
	int i, j, n[2];
	PP s[2], g[2];
	double ans, x;

	n[0] = in()-2, n[1] = in()-2;
	for (i = 0; i < 2; i++) {
		s[i].x = in(), s[i].y = in();
		g[i].x = in(), g[i].y = in();
		for (j = 0; j < n[i]; j++) p[i][j].x = in(), p[i][j].y = in();
	}

	ans = INF+100;
	for (i = 0; i < 2; i++) {
		x = dist(s+1-i, g+1-i) + calc(s+i, g+i, p[i], n[i], s+1-i, g+1-i);
		if (x < ans) ans = x;
	}

	if (ans >= INF) puts("-1");
	else printf("%.12lf\n", ans);
	return 0;
}
