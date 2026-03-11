// AOJ 0237: The Last Door
// 2018.2.21 bal4u

#include <stdio.h>
#include <string.h>
#include <math.h>
#include <ctype.h>

#define EPS 0.01
typedef struct { double x, y; } PP;
PP  t[101][5];
int n, d;

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	while (!isdigit(c)) c = getchar_unlocked();
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

void normalize()
{
	int i;
	PP a, b;
	double w;

	for (i = 0; i < n; i++) {
		a.x = t[i][1].x - t[i][0].x, a.y = t[i][1].y - t[i][0].y;
		b.x = t[i][1].x - t[i][2].x, b.y = t[i][1].y - t[i][2].y;
		if (fabs(norm(a)-norm(b)) < EPS) swap(&t[i][0], &t[i][1]);
		a.x = t[i][0].x - t[i][2].x, a.y = t[i][0].y - t[i][2].y;
		if (fabs(norm(a)-norm(b)) < EPS) swap(&t[i][0], &t[i][2]);
		if (ccw(t[i][0], t[i][1], t[i][2]) != COUNTER_CLOCKWISE) swap(&t[i][1], &t[i][2]);
		a.x = (t[i][1].x + t[i][2].x)/2, a.y = (t[i][1].y + t[i][2].y)/2;
		b.x = t[i][0].x - a.x, b.y = t[i][0].y - a.y;
		w = d / sqrt(norm(b));
		t[i][3].x = t[i][2].x + w*b.x, t[i][3].y = t[i][2].y + w*b.y;
		t[i][4].x = t[i][1].x + w*b.x, t[i][4].y = t[i][1].y + w*b.y;
	}
}

int isIntersect(PP p1, PP p2, PP p3, PP p4)
{
	return ccw(p1, p2, p3) * ccw(p1, p2, p4) <= 0 &&
		   ccw(p3, p4, p1) * ccw(p3, p4, p2) <= 0;
}

int isInside(PP p, PP *gon, int sz)
{
	int i, j;

	for (i = 0; i < sz; i++) {
		j = i+1; if (j == sz) j = 0;
		if (ccw(gon[i], gon[j], p) == CLOCKWISE ) return 0;
	}
	return 1;
}

int check(int a, int b)
{
	int i, j, ii, jj;

	for (i = 1; i <= 4; i++) if (isInside(t[a][i], t[b], 3)) return 1;
	for (i = 0; i < 3; i++) if (isInside(t[b][i], t[a]+1, 4)) return 1;
	for (i = 1; i <= 4; i++) {
		ii = i+1; if (ii == 5) ii = 1;
		for (j = 0; j < 3; j++) {
			jj = j+1; if (jj == 3) jj = 0;
			if (isIntersect(t[a][i], t[a][ii], t[b][j], t[b][jj])) return 1;
		}
	}
	return 0;
}

// SCC (Strongly connected components) library
#define MAX 101
int g_to[MAX][MAX], g_hi[MAX];
int r_to[MAX][MAX], r_hi[MAX];
int ord[MAX], gr[MAX], cnt;
char mk[MAX];

void dfs(int v) { int i;
	mk[v] = 1; for (i = 0; i < g_hi[v]; i++) if (!mk[g_to[v][i]]) dfs(g_to[v][i]); ord[cnt++] = v;
}
void rdfs(int v, int k) { int i;
	mk[v] = 1, gr[v] = k; for (i = 0; i < r_hi[v]; i++) if (!mk[r_to[v][i]]) rdfs(r_to[v][i], k);
}

int scc() {	int i, j, k, ans;
	memset(mk, 0, sizeof(mk));
	cnt = 0; for (i = 0; i < n; i++) if (!mk[i]) dfs(i);

	k = 0; memset(mk, 0, sizeof(mk));
	for (i = n-1; i >= 0; i--) if (!mk[ord[i]]) rdfs(ord[i], k++);

	ans = k;
	memset(mk, 0, sizeof(mk));
	for (i = 0; i < n; i++) for (j = 0; j < g_hi[i]; j++) {
		k = g_to[i][j];
		if (gr[i] != gr[k]) { ans -= 1 ^ mk[gr[k]], mk[gr[k]] = 1; }
	}
	return ans;
}
/// SCC above

int main()
{
	int i, j;

	while (n = in()) {
		d = in();
		for (i = 0; i < n; i++) {
			for (j = 0; j < 3; j++)	t[i][j].x = in(), t[i][j].y = in();
		}
		normalize();

		memset(g_hi, 0, sizeof(g_hi)), memset(r_hi, 0, sizeof(r_hi));
		for (i = 0; i < n; i++) for (j = 0; j < n; j++) if (i != j) {
			if (check(i, j)) g_to[i][g_hi[i]++] = j, r_to[j][r_hi[j]++] = i;
		}	
		printf("%d\n", scc());
	}
	return 0;
}
