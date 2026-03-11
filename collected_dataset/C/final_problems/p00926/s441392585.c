#include <stdio.h>
#include <stdlib.h>

#define N 1000
#define M 500

#define MAX(A,B) ((A) < (B) ? (B) : (A))

static int n;
static int m;
static int d[M][2];

int compare(const void *a, const void *b) {
  if (*(const int*)a < *(const int*)b)
    return -1;
  if (*(const int*)b < *(const int*)a)
    return 1;
  return 0;
}

static void read(void) {
  int i, sp;
  
  scanf("%d%d", &n, &m);
  for (i = 0; i < m; ++i) {
    scanf("%d%d", &d[i][0], &d[i][1]);
  }
  qsort(d, m, sizeof(int) * 2, compare);
  
  sp = 0;
  for (i = 1; i < m; ++i) {
    if (d[i][0] <= d[sp][1]) {
      d[sp][1] = MAX(d[sp][1], d[i][1]);
    } else {
      ++sp;
      d[sp][0] = d[i][0];
      d[sp][1] = d[i][1];
    }
  }
  m = sp + 1;
}

static int eval(void) {
  int pos, rval;
  int i;
  
  pos = 0;
  rval = 0;
  for (i = 0; i < m; ++i) {
    rval += d[i][1] - pos;
    rval += (d[i][1] - d[i][0]) * 2;
    pos = d[i][1];
  }
  rval += n - pos + 1;
  
  return rval;
}

int main(void) {
  read();
  printf("%d\n", eval());

  return EXIT_SUCCESS;
}