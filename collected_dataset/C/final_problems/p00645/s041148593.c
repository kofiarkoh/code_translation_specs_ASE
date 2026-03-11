#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* constant */

#define MAX_INT	((long)(1 << 30))
#define MAX_N	(5)
#define MAX_BITS	((long)(1 << (MAX_N * MAX_N)))

/* global variables */

static int n;
static unsigned char dists[MAX_BITS];
static long masks[MAX_N][MAX_N][MAX_N][MAX_N];

/* subroutines */

long make_mask(int x0, int x1, int y0, int y1) {
  int y;
  long bits, hbits;

  hbits = ((1 << (x1 - x0 + 1)) - 1) << x0;
  bits = 0;

  for (y = y0; y <= y1; y++)
    bits |= (hbits << (y * n));

  return bits;
}

int min_dist(long bits) {
  int d, id, md, x0, x1, y0, y1;
  long bits0;

  if (dists[bits] < 255) return dists[bits];

  for (id = 0; ((bits >> id) & 1) == 0; id++);

  x0 = id % n;
  y0 = id / n;
  md = MAX_INT;

  for (y1 = n - 1; y1 >= y0; y1--)
    for (x1 = n - 1; x1 >= x0; x1--) {
      bits0 = bits ^ masks[x0][x1][y0][y1];
      d = min_dist(bits0) + 1;
      if (md > d) md = d;
    }

  return (dists[bits] = md);
}

/* main */

int main(int argc, char **argv) {
  int d, i, md, x, x0, x1, y, y0, y1;
  long st;

  for (;;) {
    scanf("%d", &n);
    if (n == 0) break;

    st = 0;

    for (y = 0; y < n; y++)
      for (x = 0; x < n; x++) {
	scanf("%d", &d);
	if (d)
	  st |= (1 << (x + y * n));
      }

    memset(masks, 0, sizeof(masks));

    for (x0 = 0; x0 < n; x0++)
      for (x1 = x0; x1 < n; x1++)
	for (y0 = 0; y0 < n; y0++)
	  for (y1 = y0; y1 < n; y1++)
	    masks[x0][x1][y0][y1] = make_mask(x0, x1, y0, y1);

    memset(dists, 255, sizeof(dists));
    dists[0] = 0;

    md = min_dist(st);

    for (i = 0; i < md; i++) printf("myon");
    putchar('\n');
  }

  return 0;
}