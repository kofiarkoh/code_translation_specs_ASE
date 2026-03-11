#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* constant */

#define MAX_G	(~(1 << 31))
#define MAX_RC	(1000000)

/* global variables */

static int grids[MAX_RC];

/* main */

int main(int argc, char **argv) {
  int c, c1, c2, g, i, min_g, q, r, r1, r2, x, y;

  for (;;) {
    scanf("%d", &r);
    scanf("%d", &c);
    scanf("%d", &q);
    if ((r | c | q) == 0) break;

    for (y = 0; y < r; y++)
      for (x = 0; x < c; x++)
	scanf("%d", &(grids[y * c + x]));

    for (i = 0; i < q; i++) {
      scanf("%d", &r1);
      scanf("%d", &c1);
      scanf("%d", &r2);
      scanf("%d", &c2);

      min_g = MAX_G;

      for (y = r1; y <= r2; y++)
	for (x = c1; x <= c2; x++) {
	  g = grids[y * c + x];
	  if (min_g > g) min_g = g;
	}

      printf("%d\n", min_g);
    }
  }

  return 0;
}