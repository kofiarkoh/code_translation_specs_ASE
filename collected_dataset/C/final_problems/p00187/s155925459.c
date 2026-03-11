// Aizu Vol-1 0187: Stoning Fortune
// 2017.8.10

#include <stdio.h>
#include <math.h>

#define EPS 1e-6

typedef struct { int x, y; } PP;
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
			scanf("%d%d%d%d", &(pp[i].x), &(pp[i].y), &(pp[i + 1].x), &(pp[i + 1].y));
			if (!i && !pp[0].x && !pp[0].y && !pp[1].x && !pp[1].y) return 0;
		}
		ans = 4;
		if (!intersection(fp    , pp    , pp + 1, pp + 2, pp + 3)) goto NG;
		if (!intersection(fp + 1, pp + 2, pp + 3, pp + 4, pp + 5)) goto NG;
		if (!intersection(fp + 2, pp + 4, pp + 5, pp    , pp + 1)) goto NG;

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

int intersection(fPP *p, PP *p1, PP *p2, PP *p3, PP *p4)
{
	int t1, t2, t3, t4;
	double bo, ua;

	t1 = (p1->y - p3->y)*(p3->x - p4->x) - (p1->x - p3->x)*(p3->y - p4->y);
	t2 = (p2->y - p3->y)*(p3->x - p4->x) - (p2->x - p3->x)*(p3->y - p4->y);
	t3 = (p3->y - p1->y)*(p1->x - p2->x) - (p3->x - p1->x)*(p1->y - p2->y);
	t4 = (p4->y - p1->y)*(p1->x - p2->x) - (p4->x - p1->x)*(p1->y - p2->y);
	bo = (p4->y - p3->y)*(p2->x - p1->x) - (p4->x - p3->x)*(p2->y - p1->y);
	if (fabs(bo) <= EPS) return 0;
	ua = (p4->x - p3->x)*(p1->y - p3->y) - (p4->y - p3->y)*(p1->x - p3->x);
	p->x = p1->x + ua / bo * (p2->x - p1->x);
	p->y = p1->y + ua / bo * (p2->y - p1->y);
	if (t1*t2 >= 0 || t3*t4 >= 0) {
		if ((p1->x == p3->x && p1->y == p3->y) || (p1->x == p4->x && p1->y == p4->y) ||
			(p2->x == p3->x && p2->y == p3->y) || (p2->x == p4->x && p2->y == p4->y));
		else return 0;
	}
	return 1;
}