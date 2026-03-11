// AOJ 2641 Magic Bullet
// 2018.1.31 bal4u

#include <stdio.h>
#include <math.h>

#define EPS 1e-8

typedef struct { double x, y, z; } PP;				// Point
typedef struct { PP a, b; } LN;						// Line
typedef struct { PP p; int r; long long l; } SP;	// Sphere
SP ball[51];

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

long long inlong()
{
	long long n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

double dot(PP a, PP b)
{
	return a.x*b.x + a.y*b.y + a.z*b.z;
}

void project(PP *q, LN ln, PP p)
{
	double t;
	PP d, e;

	d.x = ln.a.x-ln.b.x, d.y = ln.a.y-ln.b.y, d.z = ln.a.z-ln.b.z;
	e.x = p.x-ln.a.x, e.y = p.y-ln.a.y, e.z = p.z-ln.a.z;
	t = dot(d, e) / dot(d, d);
	q->x = ln.a.x + d.x*t, q->y = ln.a.y + d.y*t, q->z = ln.a.z + d.z*t; 
}

double dist(PP a, PP b)
{
	double dx, dy, dz;
	dx = a.x-b.x, dy = a.y-b.y, dz = a.z-b.z;
	return sqrt(dx*dx + dy*dy + dz*dz);
}

int main()
{
	int n, q, i;
	long long ans;
	LN ln;
	PP p;

	n = in(), q = in();
	for (i = 0; i < n; i++) {
		ball[i].p.x = in(), ball[i].p.y = in(), ball[i].p.z = in();
		ball[i].r = in(), ball[i].l = inlong();
	}
	while (q--) {
		ln.a.x = in(), ln.a.y = in(), ln.a.z = in();
		ln.b.x = in(), ln.b.y = in(), ln.b.z = in();

		ans = 0;
		for (i = 0; i < n; i++) {
			project(&p, ln, ball[i].p);

			if (dist(ln.a, ln.b) >= dist(p, ln.a) &&
				dist(ln.a, ln.b) >= dist(p, ln.b) &&
				dist(p, ball[i].p) < ball[i].r + EPS) ans += ball[i].l;
		}
		printf("%lld\n", ans);
	}
	return 0;
}

