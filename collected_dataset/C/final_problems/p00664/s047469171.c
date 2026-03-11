/*
 * 1079.c: Cosmic Market
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* constant */

#define MAX_R	(50000)
#define MAX_C	(50000)
#define MAX_Q	(50000)

/* typedef */

typedef struct _od_t {
  int seq, tf;
} od_t;

/* global variables */

static od_t rows[MAX_R], cols[MAX_C];

/* main */

int main(int argc, char **argv) {
  int ai, bi, c, count, i, j, k, oi, q, r, tf;

  for (;;) {
    scanf("%d %d %d", &r, &c, &q);
    if ((r | c | q) == 0) break;

    bzero(rows, sizeof(rows));
    bzero(cols, sizeof(cols));

    for (k = 1; k <= q; k++) {
      scanf("%d %d %d", &ai, &bi, &oi);

      if (ai == 0) {
	rows[bi].seq = k;
	rows[bi].tf = (oi == 1);
      }
      else {
	cols[bi].seq = k;
	cols[bi].tf = (oi == 1);
      }
    }

    count = 0;

    for (i = 0; i < r; i++)
      for (j = 0; j < c; j++) {
	tf = (rows[i].seq >= cols[j].seq) ? rows[i].tf : cols[j].tf;
	if (tf) count++;
      }

    printf("%d\n", count);
  }

  return 0;
}