// AOJ 0237: The Last Door
// 2018.2.21 bal4u

#include <stdio.h>
#include <string.h>
#include <math.h>
#include <ctype.h>

#define MAX 102
#define EPS 1e-5

typedef struct { double x, y; } PP;
PP  t[MAX][5];
int n, d;

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
while (c <= ' ') c = getchar_unlocked();
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

#define COUNTER_CLOCKWISE	 1
#define CLOCKWISE			-1
#define ONLINE_BACK			 2
#define ONLINE_FRONT		-2
#define ON_SEG				 0

double cross(PP a, PP b) { return a.x * b.y - a.y * b.x; }
double dot(PP a, PP b) { return a.x * b.x + a.y * b.y; }
double norm(PP a) { return a.x * a.x + a.y * a.y; }

int ccw(PP p0, PP p1, PP p2)
{
	PP a, b;
	double t;

	a.x = p1.x - p0.x, a.y = p1.y - p0.y;
	b.x = p2.x - p0.x, b.y = p2.y - p0.y;
	t = cross(a, b);
	if (t >  EPS) return COUNTER_CLOCKWISE;
	if (t < -EPS) return CLOCKWISE;
	if (dot(a, b) < -EPS) return ONLINE_BACK;
	if (norm(a) < norm(b)) return ONLINE_FRONT;
	return ON_SEG;
}

void swap(PP *a, PP *b) { PP t = *a; *a = *b, *b = t; }

void normalize(PP *p)
{
	PP a, b;
	double w;

	a.x = p[1].x - p[0].x, a.y = p[1].y - p[0].y;
	b.x = p[1].x - p[2].x, b.y = p[1].y - p[2].y;
	if (fabs(norm(a)-norm(b)) < EPS) swap(&p[0], &p[1]);
	a.x = p[2].x - p[0].x, a.y = p[2].y - p[0].y;
	b.x = p[2].x - p[1].x, b.y = p[2].y - p[1].y;
	if (fabs(norm(a)-norm(b)) < EPS) swap(&p[0], &p[2]);
	if (ccw(p[0], p[1], p[2]) != COUNTER_CLOCKWISE) swap(&p[1], &p[2]);
	a.x = (p[1].x + p[2].x)/2, a.y = (p[1].y + p[2].y)/2;
	b.x = p[0].x - a.x, b.y = p[0].y - a.y, w = d / sqrt(norm(b));
	p[3].x = p[2].x + w*b.x, p[3].y = p[2].y + w*b.y;
	p[4].x = p[1].x + w*b.x, p[4].y = p[1].y + w*b.y;
}

int intersect(PP p1, PP p2, PP p3, PP p4)
{
	return ccw(p1, p2, p3) * ccw(p1, p2, p4) <= 0 &&
		   ccw(p3, p4, p1) * ccw(p3, p4, p2) <= 0;
}

int inside(PP p, PP *gon, int sz)
{
	int i, j;

	for (i = 0; i < sz; i++) { j = i+1; if (j == sz) j = 0;
		if (ccw(gon[i], gon[j], p) == CLOCKWISE ) return 0;
	}
	return 1;
}

int check(PP *a, int sa, PP *b, int sb)
{
	int i, j, ii, jj;

	for (i = 0; i < sa; i++) if (inside(a[i], b, sb)) return 1;
	for (j = 0; j < sb; j++) if (inside(b[j], a, sa)) return 1;
	for (i = 0; i < sa; i++) { ii = i+1; if (ii == sa) ii = 0;
		for (j = 0; j < sb; j++) { jj = j+1; if (jj == sb) jj = 0;
			if (intersect(a[i], a[ii], b[j], b[jj])) return 1;
		}
	}
	return 0;
}

// SCC (Strongly connected components) library
int g_to[MAX][MAX], g_hi[MAX];
int r_to[MAX][MAX], r_hi[MAX];
int ord[MAX], cmp[MAX], cnt;
char mk[MAX];

void dfs(int v) { int i;
	mk[v] = 1; for (i = 0; i < g_hi[v]; i++) if (!mk[g_to[v][i]]) dfs(g_to[v][i]); ord[cnt++] = v;
}
void rdfs(int v, int k) { int i;
	mk[v] = 1, cmp[v] = k; for (i = 0; i < r_hi[v]; i++) if (!mk[r_to[v][i]]) rdfs(r_to[v][i], k);
}

int scc() {	int i, k;
	memset(mk, 0, sizeof(mk));
	cnt = 0; for (i = 0; i < n; i++) if (!mk[i]) dfs(i);
	k = 0; memset(mk, 0, sizeof(mk));
	for (i = cnt-1; i >= 0; i--) if (!mk[ord[i]]) rdfs(ord[i], k++);
	return k;
}
/// SCC above

int main()
{
	int i, j, k, ans;

	while (n = in()) {
		d = in();
		for (i = 0; i < n; i++) {
			for (j = 0; j < 3; j++)	t[i][j].x = in(), t[i][j].y = in();
			normalize(t[i]);
		}

		memset(g_hi, 0, sizeof(g_hi)), memset(r_hi, 0, sizeof(r_hi));
		for (i = 0; i < n; i++) for (j = 0; j < n; j++) if (i != j) {
			if (check(t[i]+1, 4, t[j], 3)) {
				g_to[i][g_hi[i]++] = j, r_to[j][r_hi[j]++] = i;
			}
		}
		
		ans = scc();
		memset(mk, 0, sizeof(mk));
		for (i = 0; i < n; i++) for (j = 0; j < g_hi[i]; j++) {
			k = g_to[i][j];
			if (cmp[i] != cmp[k]) ans -= 1^mk[cmp[k]], mk[cmp[k]] = 1;
		}
		printf("%d\n", ans);
	}
	return 0;
}
