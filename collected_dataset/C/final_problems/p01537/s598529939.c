// AOJ 2423 Code Art Online
// 2018.4.8 bal4u

#include <stdio.h>
#include <string.h>
#include <math.h>

#define MAX 210
short hi[MAX], to[MAX][MAX];
char seen[MAX];
int  match[MAX];

int bpm(int u);
int bipartiteMatching(int m, int n)
{
	int u, max;

	memset(match, -1, (m+n)*sizeof(int));
	max = 0;
	for (u = 0; u < m; u++) {
		memset(seen, 0, m+n);
		if (bpm(u)) max++;
	}
	return max;
}

int bpm(int u)
{
	int i, v;

	for (i = 0; i < hi[u]; i++) {
		v = to[u][i];
		if (seen[v]) continue;
		seen[v] = 1;
		if (match[v] < 0 || bpm(match[v])) {
			match[u] = v, match[v] = u;
			return 1;
		}
	}
	return 0;
}

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

int    rn[102];
double rm[102];
char   rel[102][102];
char   assign[102], f[102];

typedef struct { double x, y; } PP;
PP p[102];

#define EPS 1e-2
#define EQ(a,b)  (fabs((a)-(b))<EPS)

double smallestCircle(int n)
{
	int  k, i, t;
	PP a = { 50, 50 };
	double move, max, d;

	move = 0.5;
	while (move > 1e-4) {
		for (t = 0; t < 50; t++) {
			max = 0;
			for (i = 0; i < n; i++) {
				if ((d = hypot(a.x-p[i].x, a.y-p[i].y)) > max) max = d, k = i;
			}
			a.x += (p[k].x - a.x)*move;
			a.y += (p[k].y - a.y)*move;
		}
		move /= 2;
	}
	return max;
}

int main()
{
	int n, m, i, j, k, l;

	n = in(), m = in();
	for (i = 0; i < n; i++) rn[i] = in();
	for (i = 0; i < m; i++) {
		k = in();
		for (j = 0; j < k; j++) p[j].x = in(), p[j].y = in();
		rm[i] = smallestCircle(k);
	}

	for (i = 0; i < m; i++) for (j = 0; j < n; j++) {
		if (EQ(rm[i], rn[j]) || rm[i] <= rn[j]) {
			rel[i][j] = 1;
			to[i][hi[i]++] = m+j;
		}
	}

	if (bipartiteMatching(m, n) != m) puts("NG");
	else {
		for (i = 0; i < m; i++) {
			for (j = 0; j < n; j++) if (!f[j] && rel[i][j]) {
				for (k = i+1; k < m; k++) hi[k] = 0;
				f[j] = 1;
				hi[i] = 1, to[i][0] = m+j;
				for (k = i+1; k < m; k++) {
					for (l = 0; l < n; l++) if (!f[l]) {
						if (rel[k][l]) to[k][hi[k]++] = m+l;
					}
					if (hi[k] == 0) { f[j] = 0; continue; }
				}

				if (bipartiteMatching(m, n) == m) {
					assign[i] = 1+j;
					break;
				}
				f[j] = 0;
			}
		}
		for (i = 0; i < m; i++) printf("%d\n", assign[i]);
	}
	return 0;
}
