#include<stdio.h>


void bsort_reverse(int *a, int size)
{
  int i;
  int change = 1;
  int tmp;

  while (change) {
    change = 0;
    for(i=0; i<(size - 1); i++) {
      if (a[i] < a[i+1]) {
        tmp = a[i];
        a[i] = a[i+1];
        a[i+1] = tmp;
        change = 1;
      }
    }// for
  }//while

  return;
}

int get_taka_one(int N, int *A, int *E)
{
  int i;

  for (i=0; i<N; i++) {
    if (E[i] == 0) continue;
    E[i] = 0;
    return A[i];
  }

  return 0;
}

int get_aoki_one(int N, int *A, int *E, int X)
{
  int i;
  int pos;
  int x = 0;
  int diff_plus;
  int diff_minus;

  for (i=0; i<N; i++) {
    if (E[i] == 0) continue;
    if (A[i] == X) {
      E[i] = 0;
      return X;
    }
    if (A[i] > X) {
      x = A[i];
      pos = i;
    }
    if (A[i] < X) {
      if (x == 0) {
        E[i] = 0;
        return A[i];
      }
      diff_plus  = x - X;
      diff_minus = X - A[i];
      if (diff_plus < diff_minus) {
        E[pos] = 0;
        return x;
      } else {
        E[i] = 0;
        return A[i];
      }
    }
  }

  if (x > 0) {
    E[pos] = 0;
  }
  return x;
}

int get_taka_result(int N, int *A, int X)
{
  int taka = 0;
  int aoki = 0;
  int E[100000]; // exist
  int i;

  for (i=0; i<N; i++) {
    E[i] = 1;
  }

  for (i=0; i<N; i++) {
    taka += get_taka_one(N, A, E);
    aoki += get_aoki_one(N, A, E, X);
  }

  return taka;
}


int main()
{
  int N;
  int Q;
  int A[100000];
  int X[100000];
  int x;
  int cnt;
  int i;
  
  cnt = scanf("%d %d", &N, &Q);

  for (i=0; i<N; i++) {
    scanf("%d", &A[i]);
  }

  bsort_reverse(A, N);

  for (i=0; i<Q; i++) {
    scanf("%d", &X[i]);
  }
  for (i=0; i<Q; i++) {
    printf("%d\n", get_taka_result(N, A, X[i]));
  }

  return 0;
}