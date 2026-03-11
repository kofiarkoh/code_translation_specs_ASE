// Aizu 1298: Separate Points
// 2017.10.30 bal4u@uu

#include <stdio.h>
#include <math.h>

#define INF 0x3fffffff
#define EPS 1e-7
#define JUST_ON 1

long long dot(int x1, int y1, int x2, int y2)
{
	return (long long)x1*x2 + (long long)y1*y2;
}

long long cross(int x1, int y1, int x2, int y2)
{
	return (long long)x1*y2 - (long long)y1*x2; 
}

int insidePolygon(int x, int y, int pn, int *px, int *py)
{
	int i, xa, ya, xb, yb, t, f;

	f = 0;
	for (i = 0; i < pn; i++) {
		xa = px[i       ] - x, ya = py[i       ] - y;
		xb = px[(i+1)%pn] - x, yb = py[(i+1)%pn] - y;
		if (fabs((double)cross(xa, ya, xb, yb)) < EPS && dot(xa, ya, xb, yb) < EPS) return 1;
		if (ya > yb) t = ya, ya = yb, yb = t, t = xa, xa = xb, xb = t;
		if (ya < EPS && EPS < yb && cross(xa, ya, xb, yb) > EPS) f = !f;
	}
	return f;
}

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

long long distance(int x1, int y1, int x2, int y2)
{
    int dx = x1 - x2;
    int dy = y1 - y2;
    return (long long)dx*dx + (long long)dy*dy;
}

int convexHull(int pn, int *x, int *y)
{
    int i, j;
    int min;
    double minang, nowang;
    long long dist;

    min = 0;
    for (i = 1; i < pn; i++) {
        if (y[i] < y[min]) min = i;
        else if (y[i] == y[min] && x[i] < x[min]) min = i;
    }

    minang = 0;
    x[pn] = x[min], y[pn] = y[min];
    for (j = 0; j < pn; j++) {
        int tx, ty;
        if (j != min) {
            tx = x[j], x[j] = x[min], x[min] = tx;
            ty = y[j], y[j] = y[min], y[min] = ty;
        }
        dist = 0x7ffffff;
        min = pn; nowang = minang; minang = 360;
        for (i = j+1; i <= pn; i++) {
            long long d = distance(x[j], y[j], x[i], y[i]);
            double rad = theta(x[j], y[j], x[i], y[i]);

            if (d == 0) continue;
            if (rad < nowang) continue;
            if (rad > nowang + EPS) {
                if (rad + EPS < minang) {
                    dist = d;
                    minang = rad;
                    min = i;
                } else if (fabs(rad-minang) < EPS && d > EPS && d > dist) {
                    dist = d;
                    minang = rad;
                    min = i;
                }
            } else if (fabs(rad-nowang) < EPS && d > EPS &&
                       fabs(rad-minang) > EPS || d > dist) {
                dist = d;
                minang = rad;
                min = i;
            }
        }
        if (min == pn) break;
    }
    return j+1;
}

int isPointOnSegment(int x, int y, int x1, int y1, int x2, int y2)
{
	int d;
	if (x1 > x2) { d = x1, x1 = x2, x2 = d; d = y1, y1 = y2, y2 = d; }
	return x1 <= x && x <= x2 && ((y1 <= y2 && y1 <= y && y <= y2) ||
		(y1 > y2 && y2 <= y && y <= y1))
		&& (long long)(y - y1)*(x2 - x1) == (long long)(y2 - y1)*(x - x1);
}

int intersection(int x1, int y1, int x2, int y2, int x3, int y3, int x4, int y4)
{
	long long t1, t2, t3, t4;

	if ((long long)(y2 - y1)*(x4 - x3) == (long long)(y4 - y3)*(x2 - x1)) return 0;

	t1 = (long long)(y1 - y3)*(x3 - x4) - (long long)(x1 - x3)*(y3 - y4);
	t2 = (long long)(y2 - y3)*(x3 - x4) - (long long)(x2 - x3)*(y3 - y4);
	t3 = (long long)(y3 - y1)*(x1 - x2) - (long long)(x3 - x1)*(y1 - y2);
	t4 = (long long)(y4 - y1)*(x1 - x2) - (long long)(x4 - x1)*(y1 - y2);

	if (t1*t2 < 0 && t3*t4 < 0) return 1;
	t1 = isPointOnSegment(x1,y1,x3,y3,x4,y4), t2 = isPointOnSegment(x2,y2,x3,y3,x4,y3);
	t3 = isPointOnSegment(x3,y3,x1,y1,x2,y2), t4 = isPointOnSegment(x4,y4,x1,y1,x2,y2);
	return t1 || t2 || t3 || t4;
}

int xb[102], yb[102], n;
int xw[102], yw[102], m;

int main()
{
	int i, sb, sw;
	int xbx0, ybx0, xbx9, ybx9, xby0, yby0, xby9, yby9;
	int xwx0, ywx0, xwx9, ywx9, xwy0, ywy0, xwy9, ywy9;

	while (scanf("%d%d", &n, &m) && (n > 0 || m > 0)) {
		for (i = 0; i < n; i++) scanf("%d%d", xb+i, yb+i);
		sb = convexHull(n, xb, yb);
		xb[sb] = xb[0], yb[sb] = yb[0];

		for (i = 0; i < m; i++) scanf("%d%d", xw+i, yw+i);
		sw = convexHull(m, xw, yw);
		xw[sw] = xw[0], yw[sw] = yw[0];


		if (sb < 1 || sw < 1) goto yes;
		if (sb == 1 && sw == 1) goto yes;
		if (sb == 1 && sw == 2) {
			if(isPointOnSegment(xb[0], yb[0], xw[0], yw[0], xw[1], yw[1])) goto no;
		}
		if (sb == 2 && sw == 1) {
			if(isPointOnSegment(xw[0], yw[0], xb[0], yb[0], xb[1], yb[1])) goto no;
		}
		if (sb == 2 && sw == 2) {
			if (intersection(xb[0],yb[0],xb[1],yb[1],xw[0],yw[0],xw[1],yw[1])) goto no;
		}

		if (sw >= 3) for (i = 0; i < sb; i++)
			if (insidePolygon(xb[i], yb[i], sw, xw, yw)) goto no;
		if (sb >= 3) for (i = 0; i < sw; i++)
			if (insidePolygon(xw[i], yw[i], sb, xb, yb)) goto no;

		xbx0 = yby0 = INF, xbx9 = yby9 = -INF;
		for (i = 0; i < sb; i++) {
			if      (xb[i] < xbx0) xbx0 = xb[i], ybx0 = yb[i];
			else if (xb[i] > xbx9) xbx9 = xb[i], ybx9 = yb[i];
			if      (yb[i] < yby0) xby0 = xb[i], yby0 = yb[i];
			else if (yb[i] > yby9) xby9 = xb[i], yby9 = yb[i];
		}
		xwx0 = ywy0 = INF, xwx9 = ywy9 = -INF;
		for (i = 0; i < sw; i++) {
			if      (xw[i] < xwx0) xwx0 = xw[i], ywx0 = yw[i];
			else if (xw[i] > xwx9) xwx9 = xw[i], ywx9 = yw[i];
			if      (yw[i] < ywy0) xwy0 = xw[i], ywy0 = yw[i];
			else if (yw[i] > ywy9) xwy9 = xw[i], ywy9 = yw[i];
		}

		if (xbx0 <= xwx0 && xwx9 <= xbx9 && ywy0 <= yby0 && yby9 <= ywy9) goto no;
		if (xwx0 <= xbx0 && xbx9 <= xwx9 && yby0 <= ywy0 && ywy9 <= yby9) goto no;
		if (xwx0 <= xbx0 && xbx9 <= xwx9 && yby0 <= ywy0 && ywy9 <= yby9) goto no;
		if (xbx0 <= xwx0 && xwx9 <= xbx9 && ywy0 <= yby0 && yby9 <= ywy9) goto no;

yes:	puts("YES"); continue;
no:		puts("NO");
	}
	return 0;
}