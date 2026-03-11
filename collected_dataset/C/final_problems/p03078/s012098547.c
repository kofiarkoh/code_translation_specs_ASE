#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <stdlib.h>

typedef unsigned long long u64_t;

typedef struct _tmp_t {
  u64_t val;
} tmp_t;

int compare(const void *a, const void *b)
{
  tmp_t *x;
  tmp_t *y;
  u64_t  X,Y;

  x = (tmp_t *)a;
  y = (tmp_t *)b;

  X = x->val;
  Y = y->val;

  if (X < Y) return 1;
  if (X > Y) return -1;

  return 0;
}

void qsort_tmp (tmp_t *array, int size)
{
  qsort(array, size, sizeof(tmp_t), compare);
}

#define SIZE 1000

tmp_t A[SIZE];
tmp_t B[SIZE];
tmp_t C[SIZE];

int main()
{
  int cnt;
  int i, X,Y,Z,K;
  int total = 0;
  int x,y,z;
  u64_t An, Bn, Cn;
  tmp_t *result;

  cnt = scanf("%d %d %d %d", &X, &Y, &Z, &K);

  for (i=0; i<X; i++) {
    cnt = scanf("%lld", &A[i].val);
  }
  for (i=0; i<Y; i++) {
    cnt = scanf("%lld", &B[i].val);
  }
  for (i=0; i<Z; i++) {
    cnt = scanf("%lld", &C[i].val);
  }

  qsort_tmp(A, X);
  qsort_tmp(B, Y);
  qsort_tmp(C, Z);

  result = calloc((X*Y*Z), sizeof(tmp_t));

  i = 0;
  for (x=0; x<X; x++) {
    for (y=0; y<Y; y++) {
      for (z=0; z<Z; z++) {
        result[i].val = (A[x].val + B[y].val + C[z].val);
        i++;
      }
    }
  }

  qsort_tmp(result, i);
        
  for (i=0; i<K; i++) {
    printf("%lld\n", result[i].val);
  }

  return 0;
}
