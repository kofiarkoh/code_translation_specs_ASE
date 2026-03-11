// Aizu Vol-1 0187: Stoning Fortune
// 2017.8.10

#include <stdio.h>
#include <math.h>

#define EPS 1e-6

typedef struct { long long x, y; } PP;
typedef struct { double x, y; } fPP;

char *kichi[] = { "dai-kichi", "chu-kichi", "kichi", "syo-kichi", "kyo" };
int std[] = { 1900000, 1000000, 100000, 0, -1 };

PP pp[6];
fPP fp[3];

double dis(fPP *p1, fPP *p2);
int intersection(fPP *p, PP *p1, PP *p2, PP *p3, PP *p4);

int main()
{
	int i, ans;
	double a, b, c, s, t;

	while (1) {
		for (i = 0; i < 6; i += 2) {
			scanf("%lld%lld%lld%lld", &(pp[i].x), &(pp[i].y), &(pp[i + 1].x), &(pp[i + 1].y));
			if (!i && !pp[0].x && !pp[0].y && !pp[1].x && !pp[1].y) return 0;
		}
		ans = 4;
		if (!intersection(fp, pp, pp + 1, pp + 2, pp + 3)) goto NG;
		if (!intersection(fp + 1, pp + 2, pp + 3, pp + 4, pp + 5)) goto NG;
		if (!intersection(fp + 2, pp + 4, pp + 5, pp, pp + 1)) goto NG;

		a = dis(fp, fp + 1), b = dis(fp + 1, fp + 2), c = dis(fp + 2, fp);
		s = (a + b + c) / 2.0;
		t = sqrt(s*(s - a)*(s - b)*(s - c));

		if (fabs(t) > EPS) {
			for (i = 0; ans == 4 && i < 4; i++) {
				if (fabs(t - std[i]) <= EPS || t >= std[i]) ans = i;
			}
		}
	NG:		puts(kichi[ans]);
	}
	return 0;
}

double dis(fPP *p1, fPP *p2)
{
	double x = p1->x - p2->x, y = p1->y - p2->y;
	return sqrt(x*x + y*y);
}

int isPointOnSegment(PP *p, PP *p1, PP *p2)
{
	long long x1 = p1->x, y1 = p1->y, x2 = p2->x, y2 = p2->y;
	long long d;
	if (x1 > x2) { d = x1, x1 = x2, x2 = d; d = y1, y1 = y2, y2 = d; }
	return x1 <= p->x && p->x <= x2 &&
		((y1 <= y2 && y1 <= p->y && p->y <= y2) ||
		(y1 > y2 && y2 <= p->y && p->y <= y1))
		&& (p->y - y1)*(x2 - x1) == (y2 - y1)*(p->x - x1);
}

int intersection(fPP *p, PP *p1, PP *p2, PP *p3, PP *p4)
{
	long long t1, t2, t3, t4;
	long long bo, ua;

	if ((p2->y - p1->y)*(p4->x - p3->x) == (p4->y - p3->y)*(p2->x - p1->x)) return 0;

	t1 = (p1->y - p3->y)*(p3->x - p4->x) - (p1->x - p3->x)*(p3->y - p4->y);
	t2 = (p2->y - p3->y)*(p3->x - p4->x) - (p2->x - p3->x)*(p3->y - p4->y);
	t3 = (p3->y - p1->y)*(p1->x - p2->x) - (p3->x - p1->x)*(p1->y - p2->y);
	t4 = (p4->y - p1->y)*(p1->x - p2->x) - (p4->x - p1->x)*(p1->y - p2->y);

	if (t1*t2 < 0 && t3*t4 < 0) {
		// ??????????????§??????
		bo = (p4->y - p3->y)*(p2->x - p1->x) - (p4->x - p3->x)*(p2->y - p1->y);
		ua = (p4->x - p3->x)*(p1->y - p3->y) - (p4->y - p3->y)*(p1->x - p3->x);
		p->x = p1->x + (double)ua / bo * (p2->x - p1->x);
		p->y = p1->y + (double)ua / bo * (p2->y - p1->y);
		return 1;
	}
	t1 = isPointOnSegment(p1, p3, p4), t2 = isPointOnSegment(p2, p3, p4);
	if (t1) p->x = p1->x, p->y = p1->y;
	else if (t2) p->x = p2->x, p->y = p2->y;
	t3 = isPointOnSegment(p3, p1, p2), t4 = isPointOnSegment(p4, p1, p2);
	if (t3) p->x = p3->x, p->y = p3->y;
	else if (t4) p->x = p4->x, p->y = p4->y;
	return t1 || t2 || t3 || t4;
}