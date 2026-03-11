// AOJ CGL_2_D Distance
// 2018.5.2 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef struct { double x, y; } PP;
typedef struct { PP s, e; } SEG, LINE;

#define INF			1e8
#define EPS			1e-8
#define EQ(a,b)		(fabs((a)-(b))<EPS)
#define PPeQ(a,b)	(EQ(a.x,b.x)&&EQ(a.y,b.y))

int dcmp(double x) { if (fabs(x) < EPS) return 0; return x <= 0 ? -1 : 1; }
PP vset(double x, double y) { PP r; r.x = x, r.y = y; return r; }
PP vadd(PP p1, PP p2) { PP r; r.x = p1.x + p2.x, r.y = p1.y + p2.y; return r; }
PP vsub(PP p1, PP p2) { PP r; r.x = p1.x - p2.x, r.y = p1.y - p2.y; return r; }
PP vsmul(PP p, double k) { PP r; r.x = p.x * k, r.y = p.y * k; return r; }
PP vmul(PP p1, PP p2) { PP r;
	r.x = p1.x * p2.x - p1.y * p2.y, r.y = p1.x * p2.y + p1.y * p2.x; return r; }
double vabs(PP a) { return hypot(a.x, a.y); }
double dist(PP p1, PP p2) {	return hypot(p1.x-p2.x, p1.y-p2.y); };
double cross(PP a, PP b) { return a.x * b.y - a.y * b.x; }
double dot(PP a, PP b) { return a.x * b.x + a.y * b.y; }
double norm(PP a) { return a.x * a.x + a.y * a.y; }
PP dir(SEG s) { return vsub(s.e, s.s); }

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()
{
	int n = 0, c = gc();
	if (c == '-') {	c = gc();
		do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
		return -n;
	}
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int ccw(PP p0, PP p1, PP p2)
{
	PP a, b;
	double t;

	a = vsub(p1, p0), b = vsub(p2, p0), t = cross(a, b);
	if (t >  EPS) return 1;	if (t < -EPS) return -1;
	if (dot(a, b) < -EPS) return 2;	if (norm(a) < norm(b)) return -2;
	return 0;
}

int is_intersectSS(SEG s1, SEG s2)
{
	return ccw(s1.s, s1.e, s2.s) * ccw(s1.s, s1.e, s2.e) <= 0 &&
		   ccw(s2.s, s2.e, s1.s) * ccw(s2.s, s2.e, s1.e) <= 0;
}

double distanceLP(SEG s, PP p)
{
	return fabs(cross(vsub(s.e, s.s), vsub(p, s.s)) / vabs(vsub(s.e, s.s)));
}

double distanceSP(SEG s, PP p)
{
	if (dot(vsub(s.e, s.s), vsub(p, s.s)) < 0) return vabs(vsub(p, s.s));
	if (dot(vsub(s.s, s.e), vsub(p, s.e)) < 0) return vabs(vsub(p, s.e));
	return distanceLP(s, p);
}

double distanceSS(SEG s1, SEG s2)
{
	double r, x;
	if (is_intersectSS(s1, s2)) return 0;
	r = distanceSP(s1, s2.s);
	x = distanceSP(s1, s2.e); if (x < r) r = x;
	x = distanceSP(s2, s1.s); if (x < r) r = x;
	x = distanceSP(s2, s1.e); if (x < r) r = x;
	return r;
}

int main()
{
	int q, x, y;
	PP p0, p1, p2, p3;
	SEG s1, s2;

	q = in();
	while (q--) {
		x = in(), y = in(), p0 = vset(x, y);
		x = in(), y = in(), p1 = vset(x, y);
		x = in(), y = in(), p2 = vset(x, y);
		x = in(), y = in(), p3 = vset(x, y);
		s1.s = p0, s1.e = p1;
		s2.s = p2, s2.e = p3;
		printf("%.10lf\n", distanceSS(s1, s2));
	}
	return 0;
}

