/*
 * 1023.c: Amazing Graze
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* constant */

#define MAX_AN	(100000)
#define MAX_CRD	(10000)
#define NUM_DIV	(200)

/* typedef */

typedef struct _point_t {
  int x, y;
  struct _point_t *next;
} point_t;

/* main */

int main(int argc, char **argv) {
  int i;
  int an, bn, r, r4, rr4;
  int sql;
  int ax, ay, bx, by, dx, dy;
  int xi, yi, id, bxi, byi, dxi, dyi;
  long power;
  point_t *pt, *pt0;
  static point_t pts[MAX_AN], *ars[NUM_DIV * NUM_DIV];
  static int npt;

  sql = MAX_CRD / NUM_DIV;

  for (;;) {
    scanf("%d", &an);
    scanf("%d", &bn);
    scanf("%d", &r);
    if ((an | bn | r) == 0) break;

    r4 = r * 4;
    rr4 = r4 * r4;

    bzero(ars, sizeof(point_t *) * NUM_DIV * NUM_DIV);
    npt = 0;

    for (i = 0; i < an; i++) {
      scanf("%d %d", &ax, &ay);

      xi = ax / sql;
      yi = ay / sql;

      id = yi * NUM_DIV + xi;
      pt = &(pts[npt++]);
      pt->x = ax;
      pt->y = ay;
      pt->next = ars[id];
      ars[id] = pt;
    }

    power = 0;

    for (i = 0; i < bn; i++) {
      scanf("%d %d", &bx, &by);

      bxi = bx / sql;
      byi = by / sql;

      for (dxi = -1; dxi <= 1; dxi++) {
	xi = bxi + dxi;
	if (xi < 0 || xi >= NUM_DIV) continue;

	for (dyi = -1; dyi <= 1; dyi++) {
	  yi = byi + dyi;
	  if (yi < 0 || yi >= NUM_DIV) continue;

	  pt = ars[yi * NUM_DIV + xi];
	  while (pt != NULL) {
	    dx = pt->x - bx;
	    dy = pt->y - by;
	    if (dx * dx + dy * dy <= rr4) power++;
	    pt = pt->next;
	  }
	}
      }
    }

    printf("%ld\n", power);
  }

  return 0;
}