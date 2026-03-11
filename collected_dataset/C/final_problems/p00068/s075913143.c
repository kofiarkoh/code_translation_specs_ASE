#include <stdio.h>
#include <math.h>
#define GRADIENT_MIN (0)
#define GRADIENT_MAX (1)

int find(size_t n, double list[100][2], double p[2], int max) {
	int i, midx;
	double dx, dy, m, g;

	m = copysign(INFINITY, max ? -1.0 : 1.0);
	for (i = 0; i < n; i++) {
		if (isnan(list[i][0]) || list[i][0] < p[0])
			continue;

		dx = p[0] - list[i][0];
		dy = p[1] - list[i][1];
		g = (dx != 0.0) ? (dy / dx) : copysign(INFINITY, -dy);

		if (max ? (m < g) : (m > g)) {
			m = g;
			midx = i;
		}
	}
	p[0] = list[midx][0];
	p[1] = list[midx][1];
	return midx;
}

void input(size_t n, double list[100][2], double p0[2], double p1[2]) {
	int i, left;

	p0[0] = 1001.0;
	p1[0] = -1001.0;
	for (i = 0; i < n; i++) {
		scanf(" %lf,%lf ", list[i], list[i]+1);
		if (p0[0] > list[i][0]) {
			p0[0] = list[i][0];
			p0[1] = list[i][1];
			left = i;
		}
		if (p1[0] < list[i][0]) {
			p1[0] = list[i][0];
			p1[1] = list[i][1];
		}
	}
	list[left][0] = list[left][1] = NAN;
}

void rm_vertex(size_t n, double list[100][2], double p0[2], double p1[2]) {
	int i;
	double p[2];

	p[0] = p0[0];
	p[1] = p0[1];
	while (1) {
		i = find(n, list, p, GRADIENT_MAX);
		if (p[0] == p1[0] && p[1] == p1[1])
			break;
		list[i][0] = list[i][1] = NAN;
	}
	p[0] = p0[0];
	p[1] = p0[1];
	while (1) {
		i = find(n, list, p, GRADIENT_MIN);
		if (p[0] == p1[0] && p[1] == p1[1])
			break;
		list[i][0] = list[i][1] = NAN;
	}
}

int solve(size_t n) {
	double list[100][2], p0[2], p1[2];
	int i, count;

	input(n, list, p0, p1);
	rm_vertex(n, list, p0, p1);

	for (i = count = 0; i < n; i++) {
		if (!isnan(list[i][0]))
			count++;
	}
	return count - 1;
}

int main(void) {
	int n;

	while (1) {
		scanf(" %d ", &n);
		if (!n)
			break;

		printf("%d\n", solve(n));
	}
	return 0;
}