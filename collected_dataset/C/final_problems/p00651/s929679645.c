// AOJ 1066 Legend of Storia
// 2018.3.21 bal4u

#include <stdio.h>
#include <math.h>

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

#define INF         1e8
#define EPS			1e-6
#define EQ(a,b)		(fabs((a)-(b))<EPS)

typedef struct { double x, y; } PP;
typedef struct { PP s, e; } SEG, LINE;
typedef struct { double x, y, r; } C;

PP vset(double x, double y) { PP r; r.x = x, r.y = y; return r; }
PP vadd(PP p1, PP p2) { PP r; r.x = p1.x + p2.x, r.y = p1.y + p2.y; return r; }
PP vsub(PP p1, PP p2) { PP r; r.x = p1.x - p2.x, r.y = p1.y - p2.y; return r; }
PP vmul(PP p1, PP p2) { PP r;
	r.x = p1.x * p2.x - p1.y * p2.y, r.y = p1.x * p2.y + p1.y * p2.x; return r; }
double cross(PP a, PP b) { return a.x * b.y - a.y * b.x; }
double dot(PP a, PP b) { return a.x * b.x + a.y * b.y; }
double norm(PP a) { return a.x * a.x + a.y * a.y; }
double vabs(PP a) { return hypot(a.x, a.y); }
double dist(PP p1, PP p2) {	return vabs(vsub(p1, p2)); }
int ccw(PP p0, PP p1, PP p2)
{
	PP a, b; double t;
	a = vsub(p1, p0), b = vsub(p2, p0);
	t = cross(a, b);
	if (t >  EPS) return 1; if (t < -EPS) -1;
//	if (dot(a, b) < -EPS) return 2; if (norm(a) < norm(b)) return -2;  //本問題では必要なし
	return 0;
}

// 三点 prev-now, now-next で形成する２本の線分間の角度。点の順序はccw()に合わせた
double angle(PP now, PP prev, PP next)
{
  double a = vabs(vsub(next, prev));
  double b = vabs(vsub(now, prev));
  double c = vabs(vsub(now, next));
  return acos((b*b+c*c-a*a)/(2*b*c));
}

// 円と円の交点(２つ、または１つか、ゼロ)
int intersectCC(PP *p1, PP *p2, C c1, C c2)
{
	double x1 = c1.x, y1 = c1.y, r1 = c1.r;
	double x2 = c2.x, y2 = c2.y, r2 = c2.r;
	double d = dist(vset(x1, y1), vset(x2, y2));
	double w, n, x, y, p, q;
	double u, v;

	u = r1+r2, v = fabs(r1-r2);

	if ((fabs(d-u) > EPS && d >= u) || fabs(v-d) > EPS && d <= v) return 0;
	w = ((r1*r1 - r2*r2)/d + d) / 2;
	n = sqrt(r1*r1 - w*w);
	x = x1 + (x2-x1)*w / d;
	y = y1 + (y2-y1)*w / d;
	p = (y2-y1)*n / d;
	q = (x2-x1)*n / d;

	p1->x = x+p, p2->x = x-p;
	p1->y = y-q; p2->y = y+q;
	if (fabs(d-u) <= EPS) return 1;
	if (d == r1+r2) return 1;
	return 2;
}

C c0;
PP p[2][27];

double minAngle(int *id, int base, int n, PP *p)
{
	int i;
	double a, ans;
	C c;
	PP p1, p2;

	ans = INF, *id = -1;
	c.x = p[base].x, c.y = p[base].y;
	for (i = 0; i < n; i++) if (i != base) {
		c.r = dist(p[i], p[base]);
		intersectCC(&p1, &p2, c0, c);
		if (ccw(p[base], p[i], p1) < 0) p1 = p2;
		a = angle(p[base], p[i], p1);
		if (a < ans) ans = a, *id = i;
	}
	return ans;
}

int main()
{
	int N, R, Q, i, k1, k2, z, nz;
	int x, y;
	double a;
	PP t;

	while (N = in()) {
		R = in(), Q = in();
		c0.x = 0, c0.y = 0, c0.r = R;
		z = -1;
		for (i = 0; i < N; i++) {
			p[0][i].x = x = in(), p[0][i].y = y = in();
			if (z < 0 && x*x + y*y == R*R) z = i;
		}

		for (k1 = 0, k2 = 1; Q--; k1 = k2, k2 = !k2) {
			a = minAngle(&nz, z, N, p[k1]);
			for (i = 0; i < N; i++) if (i != z) {
				t = vmul(vsub(p[k1][i], p[k1][z]), vset(cos(a), sin(a)));
				p[k2][i] = vadd(p[k1][z], t);
			}
			p[k2][z] = p[k1][z];
			printf("%.12lf %.12lf\n", p[k2][nz].x, p[k2][nz].y);
			z = nz;
		}
	}
	return 0;
}
