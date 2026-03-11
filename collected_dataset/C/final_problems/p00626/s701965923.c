/*
 * 1040.c: Chocolate with Heart Marks
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* constant */

#define MAX_INT	(1 << 29)

#define MAX_H	(12)
#define MAX_W	(12)
#define MAX_NTS	(6)

/* typedef */

typedef int mat_t[MAX_H * MAX_W][MAX_H * MAX_W];
typedef int opt_t[1 << MAX_NTS][MAX_H * MAX_W];

/* global variables */

static int ts[MAX_NTS];
static mat_t gmat, dmat;
static opt_t opt;

/* subroutines */

int minimum_steiner_tree(int nts, int h, int w) {
  int ans, d, e, i, j, k, n, p, q, s;

  if (nts <= 1) return 0;

  n = h * w;

  for (i = 0; i < n; i++)
    for (j = 0; j < n; j++)
      dmat[i][j] = (i == j) ? 0 : gmat[i][j];

  for (k = 0; k < n; k++)
    for (i = 0; i < n; i++)
      for (j = 0; j < n; j++) {
	d = dmat[i][k] + dmat[k][j];
	if (dmat[i][j] > d) dmat[i][j] = d;
      }

  for (s = 0; s < (1 << nts); s++)
    for (p = 0; p < n; p++)
      opt[s][p] = MAX_INT;

  for (p = 0; p < nts; p++)
    for (q = 0; q < n; q++)
      opt[1 << p][q] = dmat[ts[p]][q];

  for (s = 1; s < (1 << nts); s++) {
    if ((s & (s - 1)) == 0) continue;

    for (p = 0; p < n; p++)
      for (e = 0; e < s; e++)
        if ((e | s) == s) {
	  d = opt[e][p] + opt[s - e][p];
          if (opt[s][p] > d) opt[s][p] = d;
	}

    for (p = 0; p < n; p++)
      for (q = 0; q < n; q++) {
	d = opt[s][q] + dmat[q][p];
        if (opt[s][p] > d) opt[s][p] = d;
      }
  }

  ans = MAX_INT;

  for (s = 0; s < (1 << nts); s++)
    for (q = 0; q < n; q++) {
      d = opt[s][q] + opt[(1 << nts) - 1 - s][q];
      if (ans > d) ans = d;
    }

  return ans;
}

/* main */

int main(int argc, char **argv) {
  int d, id, i, j, h, w, hw, nts, x, y;

  for (;;) {
    scanf("%d", &h);
    scanf("%d", &w);
    if (h == 0 && w == 0) break;

    hw = h * w;
    nts = 0;

    for (i = 0; i < hw; i++)
      for (j = 0; j < hw; j++)
	gmat[i][j] = MAX_INT;

    for (y = 0; y < h; y++)
      for (x = 0; x < w; x++) {
	id = y * w + x;

	scanf("%d", &d);
	if (d == 1)
	  ts[nts++] = id;

	if (x > 0)
	  gmat[id][id - 1] = gmat[id - 1][id] = 1;
	if (x < w - 1)
	  gmat[id][id + 1] = gmat[id + 1][id] = 1;
	if (y > 0)
	  gmat[id][id - w] = gmat[id - w][id] = 1;
	if (y < h - 1)
	  gmat[id][id + w] = gmat[id + w][id] = 1;
      }

    printf("%d\n", hw - minimum_steiner_tree(nts, h, w) - 1);
  }

  return 0;
}