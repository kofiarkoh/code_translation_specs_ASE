#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* constant */

#define MAX_PN	(100)
#define MAX_SN	(10)

/* typedef */

typedef struct _pt_t {
  int x, y;
} pt_t;

/* global variables */

static int tbl[MAX_PN][MAX_SN];
static pt_t vars[MAX_PN * MAX_SN];
static char fixed[MAX_PN * MAX_SN];
static int hnvs[MAX_PN], vnvs[MAX_SN];

/* subroutines */

/* main */

int main(int argc, char **argv) {
  int changed, i, j, k, nv, nvars, pn, sn, sum, x, x0, y, y0;
  char line[1024];

  for (;;) {
    scanf("%d", &pn);
    if (pn == 0) break;

    scanf("%d", &sn);

    bzero(hnvs, sizeof(hnvs));
    bzero(vnvs, sizeof(vnvs));
    nvars = 0;

    for (y = 0; y <= pn; y++)
      for (x = 0; x <= sn; x++) {
	scanf("%s", line);
	if (line[0] == '?') {
	  tbl[y][x] = 0;
	  vars[nvars].x = x;
	  vars[nvars].y = y;
	  fixed[nvars] = 0;
	  nvars++;
	  hnvs[y]++;
	  vnvs[x]++;
	}
	else
	  tbl[y][x] = atoi(line);
      }

    nv = nvars;
    changed = 1;

    while (nv > 0 && changed) {
      changed = 0;

      for (i = 0; i < nvars; i++) {
	x = vars[i].x;
	y = vars[i].y;

	if (fixed[i]) continue;

	if (hnvs[y] == 1) {
	  if (x == sn) {
	    sum = 0;
	    for (j = 0; j < sn; j++) sum += tbl[y][j];
	    tbl[y][x] = sum;
	  }
	  else {
	    sum = 0;
	    for (j = 0; j < sn; j++)
	      if (j != x) sum += tbl[y][j];
	    tbl[y][x] = tbl[y][sn] - sum;
	  }

	  nv--;
	  hnvs[y]--;
	  vnvs[x]--;
	  fixed[i] = 1;
	  changed = 1;
	}

	if (fixed[i]) continue;

	if (vnvs[x] == 1) {
	  if (y == pn) {
	    sum = 0;
	    for (j = 0; j < pn; j++) sum += tbl[j][x];
	    tbl[y][x] = sum;
	  }
	  else {
	    sum = 0;
	    for (j = 0; j < sn; j++)
	      if (j != y) sum += tbl[j][x];
	    tbl[y][x] = tbl[pn][x] - sum;
	  }

	  nv--;
	  hnvs[y]--;
	  vnvs[x]--;
	  fixed[i] = 1;
	  changed = 1;
	}
      }
    }

    if (nv > 0)
      puts("NO");
    else {
      for (i = 0; i < nvars; i++) {
	x = vars[i].x;
	y = vars[i].y;
	printf("%d\n", tbl[y][x]);
      }
    }

    putchar('\n');
  }

  return 0;
}