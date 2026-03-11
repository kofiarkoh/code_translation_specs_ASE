#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>

/* constant */

#define MAX_W   (7)
#define MAX_H   (7)
#define MAX_WH  (MAX_W * MAX_H)

/* typedef */

/* global variables */

static int direcs[4][2] = {{1, 0}, {0, -1}, {-1, 0}, {0, 1}};
static int dnums[6] = {3, 5, 6, 9, 10, 12};
static int ds[16][2] = {
  {-1, -1}, {-1, -1}, {-1, -1}, {0, 1}, {-1, -1}, {0, 2}, {1, 2}, {-1, -1},
  {-1, -1}, {0, 3}, {1, 3}, {-1, -1}, {2, 3}, {-1, -1}, {-1, -1}, {-1, -1},
};

static int w, h;

static int brds[MAX_H][MAX_W];
static int cells[MAX_H][MAX_W];

/* subroutines */

void print_rect(int rect[MAX_H][MAX_W]) {
  int x, y;

  for (y = 0; y < h; y++) {
    for (x = 0; x < w; x++) printf(" %d", rect[y][x]);
    putchar('\n');
  }
}

int check_cell(int x, int y, int *cs) {
  int c0, cn, d, i, ngds, okds, x0, y0;

  ngds = okds = 0;

  for (d = 0; d < 4; d++) {
    x0 = x + direcs[d][0];
    y0 = y + direcs[d][1];

    if (x0 < 0 || x0 >= w || y0 < 0 || y0 >= h || brds[y0][x0] == 1) {
      ngds |= (1 << d);
      continue;
    }

    c0 = cells[y0][x0];
    /*printf("  cells[%d,%d]=%d\n", x0, y0, c0);*/

    if (c0 > 0) {
      if (c0 & (1 << (d ^ 2)))
        okds |= (1 << d);
      else
        ngds |= (1 << d);
    }
  }
  /*printf("[%d,%d]: okds=%d,ngds=%d\n", x, y, okds, ngds);*/

  for (i = cn = 0; i < 6; i++) {
    c0 = dnums[i];
    if ((okds & c0) == okds && (ngds & c0) == 0)
      cs[cn++] = c0;
  }

  return cn;
}

int one_circle() {
  int d, d0, *ds0, x, y;
  static int visited[MAX_H][MAX_W];

  for (y = 0; y < h; y++)
    for (x = 0; x < w; x++) visited[y][x] = brds[y][x];

  x = y = 0;
  while (cells[y][x] <= 0)
    if (++x >= w) {
      x = 0;
      y++;
    }

  d = ds[cells[y][x]][0];

  while (! visited[y][x]) {
    visited[y][x] = 1;
    x += direcs[d][0];
    y += direcs[d][1];
    ds0 = ds[cells[y][x]];
    d0 = (d ^ 2);

    if (d0 == ds0[0])
      d = ds0[1];
    else if (d0 == ds0[1])
      d = ds0[0];
    else
      return 0;
  }

  for (y = 0; y < h; y++)
    for (x = 0; x < w; x++)
      if (! visited[y][x]) return 0;

  return 1;
}

int check_rec(int k, int bn, int blanks[][2]) {
  int cn, i, j, x, y;
  int cs[6];

  if (k >= bn) return one_circle();

  x = blanks[k][0];
  y = blanks[k][1];
  cn = check_cell(x, y, cs);

  for (i = 0; i < cn; i++) {
    cells[y][x] = cs[i];
    if (check_rec(k + 1, bn, blanks)) return 1;
    cells[y][x] = 0;
  }

  return 0;
}


/* main */

int main(int argc, char **argv) {
  int b, bn, changed, cn, ng, x, y;
  int cs[6], blanks[MAX_WH][2];

  for (;;) {
    scanf("%d %d", &w, &h);
    if (w == 0 && h == 0) break;

    bn = 0;
    for (y = 0; y < h; y++)
      for (x = 0; x < w; x++) {
        scanf("%d", &b);
        brds[y][x] = b;
        if (b == 0) {
          bn++;
          cells[y][x] = 0;
        }
        else
          cells[y][x] = -1;
      }

    if (0) {
      print_rect(brds);
      print_rect(cells);
    }

    if (bn == 0) {
      puts("No");
      continue;
    }

    changed = 1;
    ng = 0;

    while (changed) {
      changed = 0;

      for (y = 0; y < h; y++) {
        for (x = 0; x < w; x++) {
          if (cells[y][x] == 0) {
            cn = check_cell(x, y, cs);
            /*printf("[%d,%d]: cn=%d\n", x, y, cn);*/

            if (cn == 0) {
              ng = 1;
              break;
            }
            else if (cn == 1) {
              cells[y][x] = cs[0];
              changed = 1;
            }
          }
        }
        if (ng) break;
      }
      if (ng) break;
    }

    if (0) print_rect(cells);

    if (ng) {
      puts("No");
      continue;
    }

    bn = 0;
    for (y = 0; y < h; y++)
      for (x = 0; x < w; x++)
        if (cells[y][x] == 0) {
          blanks[bn][0] = x;
          blanks[bn][1] = y;
          bn++;
        }

    if (bn == 0)
      puts(one_circle() ? "Yes" : "No");
    else
      puts(check_rec(0, bn, blanks) ? "Yes" : "No");
  }

  return 0;
}