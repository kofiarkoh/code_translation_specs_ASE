// AOJ 2579 Disappear Drive
// 2018.5.5 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef struct { double x, y; } PP;
typedef struct { PP s, e; } SEG, LINE;
typedef struct { double x, y, r; } C;

#define INF 1e8
#define EPS 1e-8
#define EQ(a,b)		(fabs((a)-(b))<EPS)

int dcmp(double x) { if (fabs(x) < EPS) return 0; return x <= 0 ? -1 : 1; }
PP vset(double x, double y) { PP r; r.x = x, r.y = y; return r; }
PP vadd(PP p1, PP p2) { PP r; r.x = p1.x + p2.x, r.y = p1.y + p2.y; return r; }
PP vsub(PP p1, PP p2) { PP r; r.x = p1.x - p2.x, r.y = p1.y - p2.y; return r; }
PP vsmul(PP p, double k) { PP r; r.x = p.x * k, r.y = p.y * k; return r; }
PP vmul(PP p1, PP p2) { PP r;
	r.x = p1.x * p2.x - p1.y * p2.y, r.y = p1.x * p2.y + p1.y * p2.x; return r; }
double vabs(PP a) { return hypot(a.x, a.y); }
double dist(PP p1, PP p2) {	return hypot(p1.x-p2.x, p1.y-p2.y); }
double cross(PP a, PP b) { return a.x * b.y - a.y * b.x; }
double dot(PP a, PP b) { return a.x * b.x + a.y * b.y; }
double norm(PP a) { return a.x * a.x + a.y * a.y; }
PP dir(SEG s) { return vsub(s.e, s.s); }
int ccw(PP p0, PP p1, PP p2) { PP a, b;	double t;
	a = vsub(p1, p0), b = vsub(p2, p0), t = cross(a, b);
	if (t >  EPS) return 1;	if (t < -EPS) return -1;
	if (dot(a, b) < -EPS) return 2;	if (norm(a) < norm(b)) return -2;
	return 0;
}

// 線分２本の交点
PP crossPointSS(int *ok, SEG s1, SEG s2)
{
	double n, d;
	PP one = {1,1};
	n = cross(vsub(s2.s, s1.s), dir(s2));
	d = cross(dir(s1), dir(s2));
	if (EQ(d,0)) { *ok = 0; return one; } 
	*ok = 1;
	return vadd(s1.s, vsmul(dir(s1), n/d));
}

// 点と直線の距離
double distancePL(PP p, LINE ln)
{
	PP a = vsub(ln.e, ln.s);
	return fabs(cross(vsub(p, ln.s), a)) / vabs(a);
}

// 射影の交点
PP projection(PP p1, PP p2, PP p)
{
	PP p2p1 = vsub(p2, p1);
	PP pp1 = vsub(p, p1);
	return vadd(p1, vsmul(p2p1, dot(pp1, p2p1)/norm(p2p1)));
}

// 円と線分との交点数
int crossPointCS(C c, SEG line)
{
	double a;
	PP c0 = vset(c.x, c.y), p0;

	a = dist(c0, line.s);
	if (!EQ(a, c.r) && a <= c.r) return 1;
	p0 = projection(line.s, line.e, c0);
	a = dist(c0, p0);
	if (EQ(a, c.r) || a >= c.r) return 0;
	if (ccw(line.s, line.e, p0) == 0) return 2;
	return 0;
}

// 円と直線との交点数
int crossPointCL(C c, LINE line)
{
	PP c0 = vset(c.x, c.y);
	double a = distancePL(c0, line);
	if (EQ(a, c.r) || a >= c.r) return 0;
	return 2;
}

// 指定した点を通過する直線と円との接線（本問題のために一部改造）
void tangentCP(PP *po, C c, PP p)
{
	PP c0 = vset(c.x, c.y);
	double L = vabs(vsub(c0, p));
//	double M = sqrt(L*L - c.r*c.r);
	double theta = asin(c.r / L);
	PP v = vsmul(vsub(c0, p), 1/L);		// 単位ベクトル
	PP a = vset(cos(theta), sin(theta)), b = vset(cos(theta), -sin(theta));
//	po[0] = vadd(p, vsmul(vmul(v, a), M));
//	po[1] = vadd(p, vsmul(vmul(v, b), M));
	po[0] = vadd(p, vsmul(vmul(v, a), 100));
	po[1] = vadd(p, vsmul(vmul(v, b), 100));
}

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()
{
	int n = 0, c = gc();
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int D;
C c[6]; int N;
PP SP[12], GP[12]; int sz;
PP S = {25,0}, G = {25,94};

int checkCenterLine()
{
	int i, t = 0;
	LINE ln;
	ln.s = S, ln.e = G;
	for (i = 0; i < N; i++) t += crossPointCL(c[i], ln) / 2;
	return t <= D;
}

int check(PP p)
{
	int i, k, s;
	LINE ls, lg;

	if (EQ(p.x,0) || EQ(p.x,50) || p.x <= 0 || p.x >= 50) return 0;
	if (EQ(p.y,0) || EQ(p.y,94) || p.y <= 0 || p.y >= 94) return 0;

	s = 0;
	ls.s = p, ls.e = S, lg.s = p, lg.e = G;
	for (i = 0; i < N; i++) {
		k = crossPointCS(c[i], ls);
		if (k == 1) s++;
		else {
			if (k == 2) s++;
			s += crossPointCS(c[i], lg)/2;
		}
	}
	return s <= D;
}

int main()
{
	int i, j;
	double ans, d;
	PP SG;
	LINE ls, lg;

	N = in(), D = in();
	for (i = 0; i < N; i++) {
		c[i].x = in(), c[i].y = in(), c[i].r = in();
	}

	if (N == 0 || checkCenterLine()) {
		puts("94.0000000000");
		return 0;
	}

	sz = N << 1;
	for (j = 0, i = 0; i < N; i++, j += 2) {
		tangentCP(SP+j, c[i], S);
		tangentCP(GP+j, c[i], G);
	}
	ans = INF;
	for (i = 0; i < sz; i++) {
		ls.s = S, ls.e = SP[i];
		for (j = 0; j < sz; j++) {
			int k;
			lg.s = G, lg.e = GP[j];
			SG = crossPointSS(&k, ls, lg);
			if (!k) continue;
			d = dist(SG, S) + dist(SG, G);
			if (d < ans && check(SG)) ans = d;
		}
	}
	if (EQ(ans, INF)) puts("-1");
	else printf("%.10lf\n", ans);
	return 0;
}
