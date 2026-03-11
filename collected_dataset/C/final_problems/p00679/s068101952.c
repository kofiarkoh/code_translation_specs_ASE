#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/* constant */

#define MAX_N	(100000)
#define MAX_Q	(1000)
#define MAX_M	(100)

/* typedef */

typedef struct _pt_t {
  double x, y, z;
} pt_t;

/* global variables */

static int n, qn;
static pt_t wrps[MAX_N], pts[MAX_M];
static double swrps[MAX_M];

/* main */

int main(int argc, char **argv) {
  int i, k, mi, q;
  double d, dw, dx, dy, dz, min_d, sum_d;
  pt_t *ppt, *ppt0, *ppt1;

  for (;;) {
    scanf("%d %d", &n, &qn);
    if ((n | qn) == 0) break;

    for (i = 0; i < n; i++) {
      scanf("%lf %lf %lf", &(wrps[i].x), &(wrps[i].y), &(wrps[i].z));
      /*
      printf("wrps[%d]=(%lf,%lf,%lf)\n", i, wrps[i].x, wrps[i].y, wrps[i].z);
      */
    }

    for (q = 0; q < qn; q++) {
      scanf("%d", &mi);
      /*printf("mi=%d\n", mi);*/

      for (i = 0, ppt = pts; i < mi; i++, ppt++) {
	scanf("%lf %lf %lf", &(ppt->x), &(ppt->y), &(ppt->z));

	min_d = 1.0e20;
	for (k = 0; k < n; k++) {
	  dx = wrps[k].x - ppt->x;
	  dy = wrps[k].y - ppt->y;
	  dz = wrps[k].z - ppt->z;
	  d = sqrt(dx * dx + dy * dy + dz * dz);
	  if (min_d > d) min_d = d;
	}
	swrps[i] = min_d;

	/*
	printf("pts[%d]=(%lf,%lf,%lf), swrps[%d]=%lf\n",
	       i, ppt->x, ppt->y, ppt->z, i, swrps[i]);
	*/
      }

      sum_d = 0.0;
      for (i = 0, ppt0 = pts; i < mi - 1; i++, ppt0++) {
	ppt1 = ppt0 + 1;
	/*
	printf("pts[%d]=(%lf,%lf,%lf)\n", i, ppt0->x, ppt0->y, ppt0->z);
	printf("pts[%d]=(%lf,%lf,%lf)\n", i + 1, ppt1->x, ppt1->y, ppt1->z);
	*/

	dx = ppt1->x - ppt0->x;
	dy = ppt1->y - ppt0->y;
	dz = ppt1->z - ppt0->z;
	/*printf("(dx,dy,dz)=(%lf,%lf,%lf)\n", dx, dy, dz);*/

	d = sqrt(dx * dx + dy * dy + dz * dz);
	dw = swrps[i] + swrps[i + 1];
	/*printf("i=%d: d=%lf, dw=%lf\n", i, d, dw);*/

	sum_d += (d < dw) ? d : dw;
      }

      printf("%.9lf\n", sum_d);
    }
  }

  return 0;
}