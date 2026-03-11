// AOJ 2385 Shelter
// 2018.3.22 bal4u

#include <stdio.h>
#include <string.h>
#include <math.h>

// [library]

/* get an integer from stdin */
//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	if (c == '-') {	c = getchar_unlocked();
		do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
		return -n;
	}
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

/* Geometry */
typedef struct { double x, y; } PP;
typedef struct { PP s, e; } SEG, LINE;

#define EPS 1e-8
#define PI  3.1415926535897932384626433832795
#define EQ(a,b) (fabs((a)-(b))<EPS)
int dcmp(double x) { if (fabs(x) < EPS) return 0; return x <= 0 ? -1 : 1; }
PP vset(double x, double y) { PP r; r.x = x, r.y = y; return r; }
PP vadd(PP p1, PP p2) { PP r; r.x = p1.x + p2.x, r.y = p1.y + p2.y; return r; }
PP vsub(PP p1, PP p2) { PP r; r.x = p1.x - p2.x, r.y = p1.y - p2.y; return r; }
PP vsmul(PP p, double k) { PP r; r.x = p.x * k, r.y = p.y * k; return r; }
PP vmul(PP p1, PP p2) { PP r;
	r.x = p1.x * p2.x - p1.y * p2.y, r.y = p1.x * p2.y + p1.y * p2.x; return r; }
double cross(PP a, PP b) { return a.x * b.y - a.y * b.x; }
double dot(PP a, PP b) { return a.x * b.x + a.y * b.y; }
double norm(PP a) { return a.x * a.x + a.y * a.y; }
double vabs(PP a) { return hypot(a.x, a.y); }

/* ２線分間の交点 */
PP crossPointS2P(SEG a, PP bs, PP be) { double a1, a2; PP r;
	a1 = cross(vsub(be, bs), vsub(a.s, bs));
	a2 = cross(vsub(be, bs), vsub(a.e, bs));
	r.x = (a.s.x*a2 - a.e.x*a1) / (a2-a1);
	r.y = (a.s.y*a2 - a.e.y*a1) / (a2-a1);
	return r;
}

/* 凸包を直線で切断して左側をpo[]に残す */
int convex_cut(LINE u, int n, PP *p, PP *po)		// p --> po
{
	int i, d1, d2, top = 0;
    for (i = 0; i < n; i++) {
        d1 = dcmp(cross(vsub(u.e, u.s), vsub(p[i],   u.s)));
        d2 = dcmp(cross(vsub(u.e, u.s), vsub(p[i+1], u.s)));
        if (d1 >= 0) po[top++] = p[i];
        if (d1*d2 < 0) po[top++] = crossPointS2P(u, p[i], p[i+1]);
    }
	po[top] = po[0];
    return top;
}

/* 両端点a,bをもつ線分のの垂直二等分線 u */
void bisector(LINE *u, PP a, PP b)
{
	u->s = vsmul(vadd(a, b), 0.5);
	u->e = vadd(u->s, vmul(vsub(b, a), vset(0, 1)));
}

/* ポリゴンの面積 */
double polygon_area(int n, PP *p)
{
   int i;
   double s;

   if (n < 3) return 0;
   s = 0; p[n] = p[0];
   for (i = 0; i < n; i++) s += cross(p[i], p[i+1]);
   return fabs(s)/2;
}

// [\library]

PP town[101], shlter[101];
PP p[2][202];            // for convex_cut

double area(PP a, PP b, PP c)
{
	double ang, t1, t2, t3;
	b = vsub(b, a), c = vsub(c, a);
	ang = PI/2 - atan2(c.y-b.y, c.x-b.x);
	b = vmul(b, vset(cos(ang), sin(ang)));
	c = vmul(c, vset(cos(ang), sin(ang)));
	t1 = b.x, t2 = c.y/t1, t3 = b.y/t1;
	return pow(t1, 4)*(t2*(3+t2*t2)-t3*(3+t3*t3))/12;
}

int main()
{
	int M, N, i, j, k1, k2, k;
	double total, ans;
	LINE u;

	M = in(), N = in();
	for (i = 0; i < M; i++) town[i].x = in(), town[i].y = in();
	total = polygon_area(M, town);

	for (i = 0; i < N; i++) shlter[i].x = in(), shlter[i].y = in();
	ans = 0;
	for (i = 0; i < N; i++) {
		memcpy(p[0], town, sizeof(PP)*(M+1)), k = M;
		for (k1 = 0, k2 = 1, j = 0; j < N; j++) if (j != i) {
			bisector(&u, shlter[i], shlter[j]);
			k = convex_cut(u, k, p[k1], p[k2]);	
			k1 = k2, k2 = !k2;
		}
		p[k1][k] = p[k1][0];
		for (j = 0; j < k; j++) ans += area(shlter[i], p[k1][j], p[k1][j+1]);
	}
	printf("%.10lf\n", ans/total);
	return 0;
}
